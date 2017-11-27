// During tests, we disable the renderer and that causes a bunch of warnings
// See Cargo.toml for an explanation of this attribute
#![cfg_attr(any(feature = "test", test), allow(dead_code, unused_variables, unused_imports))]

use std::env;
use std::thread;
use std::process::{self, Stdio};
use std::time::Instant;
use std::cell::RefCell;
use std::sync::mpsc;

use piston_window::math;

use client;
use canvas;
use animation::{Animation, MoveAnimation, RotateAnimation, AnimationStatus};
use state::{TurtleState, DrawingState, Path};
use query::{Query, Request, DrawingCommand, Response};
use radians::{self, Radians};
use {Point, Distance, Event};

use self::DrawingCommand::*;

#[cfg(any(feature = "test", test))]
fn renderer_client(_: mpsc::Sender<Response>) -> (process::Child, thread::JoinHandle<()>) {
    let command = if cfg!(windows) {
        "dir"
    } else {
        "ls"
    };
    let mut test_proc = process::Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("test process failed to start");
    test_proc.kill().expect("test process could not be killed!");
    let handle = thread::spawn(move || {});
    (test_proc, handle)
}

#[cfg(not(any(feature = "test", test)))]
fn renderer_client(response_tx: mpsc::Sender<Response>) -> (process::Child, thread::JoinHandle<()>) {
    let current_exe = env::current_exe()
        .expect("Could not read path of the currently running executable")
        .into_os_string();
    let mut renderer_process = process::Command::new(current_exe)
        .env("RUN_TURTLE_CANVAS", "true")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("renderer process failed to start");

    let renderer_stdout = renderer_process.stdout.take()
        .expect("renderer process was not opened with stdout");
    let handle = thread::spawn(move || {
        client::run(renderer_stdout, response_tx);
    });

    (renderer_process, handle)
}

pub struct TurtleWindow {
    renderer: RefCell<process::Child>,
    thread_handle: RefCell<Option<thread::JoinHandle<()>>>,
    /// Channel for receiving responses from the rendering process
    response_channel: mpsc::Receiver<Response>,
}

impl TurtleWindow {
    pub fn new() -> TurtleWindow {
        // This needs to be called as close to the start of the program as possible
        // Since Turtle::new() is called at the beginning of many turtle programs, we do so here
        // to make sure this call occurs when it should.
        canvas::start();

        let (response_tx, response_rx) = mpsc::channel();
        let (renderer_process, handle) = renderer_client(response_tx);

        Self {
            renderer: RefCell::new(renderer_process),
            thread_handle: RefCell::new(Some(handle)),
            response_channel: response_rx,
        }
    }

    pub fn fetch_turtle(&self) -> TurtleState {
        self.send_query(Query::Request(Request::TurtleState));
        match self.wait_for_response() {
            Response::TurtleState(state) => {
                println!("{:?}", state);
                state
            },
            _ => panic!("The renderer process sent back the wrong state!"),
        }
    }

    pub fn update_turtle(&mut self, turtle: TurtleState) {
        unimplemented!();
    }

    pub fn fetch_drawing(&self) -> DrawingState {
        unimplemented!();
    }

    pub fn update_drawing(&mut self, drawing: DrawingState) {
        unimplemented!();
    }

    fn fetch_temporary_path(&self) -> Option<Path> {
        unimplemented!();
    }

    fn set_temporary_path(&mut self, path: Option<Path>) {
        unimplemented!();
    }

    /// See [`Turtle::poll_event()`](struct.Turtle.html#method.poll_event).
    pub fn poll_event(&mut self) -> Option<Event> {
        //TODO: query for an event, then read the response
        unimplemented!();
        //match self.response_channel.try_recv() {
        //    Ok(event) => Some(event),
        //    Err(TryRecvError::Empty) => None, // Do nothing
        //    // The window has been closed
        //    Err(TryRecvError::Disconnected) => self.exit_process(), // Quit
        //}
    }

    /// Begin filling the shape drawn by the turtle's movements.
    pub fn begin_fill(&mut self) {
        let fill_color = self.fetch_turtle().fill_color;
        self.send_drawing_command(BeginFill(fill_color));
    }

    /// Stop filling the current shape
    pub fn end_fill(&mut self) {
        self.send_drawing_command(EndFill);
    }

    /// Clear the turtle's drawings
    pub fn clear(&mut self) {
        assert!(self.fetch_temporary_path().is_none(),
            "bug: The temporary path was still set when the renderer was asked to clear the drawing");
        self.send_drawing_command(Clear);
    }

    /// Move the turtle to the given position without changing its heading.
    pub fn go_to(&mut self, end: Point) {
        let (start, speed, pen) = {
            let turtle = self.fetch_turtle();
            (turtle.position, turtle.speed, turtle.pen.clone())
        };

        let distance = math::square_len(math::sub(start, end)).sqrt();
        let speed = speed.to_absolute(); // px per second
        // We take the absolute value because the time is always positive, even if distance is negative
        let total_millis = (distance / speed * 1000.).abs();

        let animation = MoveAnimation {
            path: Path {start, end, pen},
            timer: Instant::now(),
            total_millis,
        };

        self.play_animation(animation);
    }

    /// Move the turtle forward by the given distance. To move backwards, use a negative distance.
    ///
    /// The turtle's motion will be animated based on the speed
    pub fn forward(&mut self, distance: Distance) {
        if distance == 0. {
            return;
        }

        let (start, speed, heading, pen) = {
            let turtle = self.fetch_turtle();
            (turtle.position, turtle.speed, turtle.heading, turtle.pen.clone())
        };
        let x = distance * heading.cos();
        let y = distance * heading.sin();
        let end = math::add(start, [x, y]);

        let speed = speed.to_absolute(); // px per second
        // We take the absolute value because the time is always positive, even if distance is negative
        let total_millis = (distance / speed * 1000.).abs();

        let animation = MoveAnimation {
            path: Path {start, end, pen},
            timer: Instant::now(),
            total_millis,
        };

        self.play_animation(animation);
    }

    /// Rotate the turtle in place by the given angle in the given direction of rotation
    pub fn rotate(&mut self, angle: Radians, clockwise: bool) {
        if angle == radians::ZERO {
            return;
        }

        let TurtleState {heading, speed, ..} = self.fetch_turtle();
        let speed = speed.to_rotation(); // radians per second
        let total_millis = angle / speed * 1000.;
        // We take the absolute value because the time is always positive, even if angle is negative
        let total_millis = total_millis.to_radians().abs();

        let animation = RotateAnimation {
            start: heading,
            delta_angle: angle,
            clockwise,
            timer: Instant::now(),
            total_millis,
        };

        self.play_animation(animation);
    }

    fn play_animation<A: Animation>(&mut self, animation: A) {
        loop {
            // We want to keep the lock for as little time as possible
            let status = {
                let mut turtle = self.fetch_turtle();
                let status = animation.advance(&mut turtle);
                self.update_turtle(turtle);
                status
            };
            match status {
                AnimationStatus::Running(path) => self.set_temporary_path(path),
                AnimationStatus::Complete(path) => {
                    if let Some(path) = path {
                        self.set_temporary_path(None);
                        self.send_drawing_command(StorePath(path));
                    }

                    break;
                },
            }
        }
    }

    fn send_drawing_command(&self, command: DrawingCommand) {
        self.send_query(Query::Drawing(command));
    }

    // During tests, we disable the renderer. That means that if we let this code run, it will
    // quit the application during the tests and make it look like everything passes.
    // We disable this code so that none of that happens.
    #[cfg(any(feature = "test", test))]
    fn send_query(&self, _: Query) {}

    #[cfg(not(any(feature = "test", test)))]
    #[inline]
    fn send_query(&self, query: Query) {
        if let Some(ref mut stdin) = self.renderer.borrow_mut().stdin {
            client::send_query(stdin, &query);
        }
        else {
            unreachable!("bug: renderer process was not opened with stdin");
        }
    }

    fn wait_for_response(&self) -> Response {
        match self.response_channel.recv() {
            Ok(response) => response,
            // The client thread has exited, that means that the renderer process has exited
            // and the window has closed
            Err(_) => self.exit_process(), // Quit
        }
    }

    /// Exits the current process with the correct error code
    ///
    /// Panics if the thread handle has already been consumed
    #[inline]
    fn exit_process(&self) -> ! {
        if let Some(handle) = self.thread_handle.borrow_mut().take() {
            // First check if the other thread panicked before it quit
            match handle.join() {
                Ok(_) => process::exit(0),
                // If this returns an error, the other thread panicked
                Err(_) => process::exit(1),
            }
        }
        else {
            unreachable!("bug: the thread handle was used but the process did not end");
        }
    }
}

#[cfg(not(any(feature = "test", test)))]
impl Drop for TurtleWindow {
    fn drop(&mut self) {
        // If the current thread is panicking, we want to abort right away
        // because otherwise there is code in the rendering thread that will call
        // process::exit(0) and then the exit code will be 0 instead of 1
        if thread::panicking() {
            process::exit(1);
        }

        // If this is just a normal ending of the main thread, we want to leave the renderer
        // running so that the user can see their drawing as long as they keep the window open
        if let Some(handle) = self.thread_handle.borrow_mut().take() {
            handle.join().unwrap_or_else(|_| {
                // If this returns an error, the other thread panicked
                process::exit(1);
            });
        }
    }
}
