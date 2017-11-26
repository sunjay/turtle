// During tests, we disable the renderer and that causes a bunch of warnings
// See Cargo.toml for an explanation of this attribute
#![cfg_attr(any(feature = "test", test), allow(dead_code, unused_variables, unused_imports))]

use std::env;
use std::thread;
use std::process;
use std::time::Instant;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use piston_window::math;

use canvas;
use animation::{Animation, MoveAnimation, RotateAnimation, AnimationStatus};
use state::{TurtleState, DrawingState, Path};
use query::DrawingCommand;
use radians::{self, Radians};
use {Point, Distance, Event};

use self::DrawingCommand::*;

/// Types that will be shared with another thread
pub type Shared<T> = Arc<RwLock<T>>;
/// Alias to help make the types more understandable without exposing as many implementation details
pub type ReadOnlyRef<'a, T> = RwLockReadGuard<'a, T>;
pub type MutableRef<'a, T> = RwLockWriteGuard<'a, T>;

/// A structure that provides read-only access to shared state
pub struct ReadOnly {
    turtle: Shared<TurtleState>,
    drawing: Shared<DrawingState>,
    /// A temporary path for use during animations
    temporary_path: Shared<Option<Path>>,
}

impl ReadOnly {
    pub fn turtle(&self) -> ReadOnlyRef<TurtleState> {
        self.turtle.read().expect("bug: Lock was poisoned")
    }

    pub fn drawing(&self) -> ReadOnlyRef<DrawingState> {
        self.drawing.read().expect("bug: Lock was poisoned")
    }

    pub fn temporary_path(&self) -> ReadOnlyRef<Option<Path>> {
        self.temporary_path.read().expect("bug: Lock was poisoned")
    }
}

pub struct TurtleWindow {
    thread_handle: Option<thread::JoinHandle<()>>,
    /// Channel for sending drawing commands to the renderer thread
    drawing_channel: mpsc::Sender<DrawingCommand>,
    /// Channel for receiving events from the rendering thread
    events_channel: mpsc::Receiver<Event>,

    turtle: Shared<TurtleState>,
    drawing: Shared<DrawingState>,
    /// A temporary path for use during animations
    temporary_path: Shared<Option<Path>>,
}

impl TurtleWindow {
    pub fn new() -> TurtleWindow {
        // If this environment variable is present, this process is hijacked (no other code runs).
        // We run the renderer loop and then immediately exit.
        if env::var("RUN_TURTLE_CANVAS").unwrap_or_else(|_| "".to_owned()) == "true" {
            canvas::run();
            unreachable!("Renderer loop did not exit after finishing");
        }

        let (drawing_tx, drawing_rx) = mpsc::channel();
        let (events_tx, events_rx) = mpsc::channel();

        let mut turtle_window = Self {
            thread_handle: None,
            drawing_channel: drawing_tx,
            events_channel: events_rx,
            turtle: Arc::new(RwLock::new(TurtleState::default())),
            drawing: Arc::new(RwLock::new(DrawingState::default())),
            temporary_path: Arc::new(RwLock::new(None)),
        };

        let read_only = turtle_window.read_only();
        let handle = thread::spawn(move || {
        });

        turtle_window.thread_handle = Some(handle);
        turtle_window
    }

    /// Provide a read-only version of the state
    pub fn read_only(&self) -> ReadOnly {
        ReadOnly {
            turtle: Arc::clone(&self.turtle),
            drawing: Arc::clone(&self.drawing),
            temporary_path: Arc::clone(&self.temporary_path),
        }
    }

    /// Provides read-only access to the turtle state
    pub fn turtle(&self) -> ReadOnlyRef<TurtleState> {
        self.turtle.read().expect("bug: Lock was poisoned")
    }

    /// Provides mutable access to the turtle state
    pub fn turtle_mut(&mut self) -> MutableRef<TurtleState> {
        self.turtle.write().expect("bug: Lock was poisoned")
    }

    /// Provides read-only access to the drawing
    pub fn drawing(&self) -> ReadOnlyRef<DrawingState> {
        self.drawing.read().expect("bug: Lock was poisoned")
    }

    /// Provides mutable access to the drawing
    pub fn drawing_mut(&mut self) -> MutableRef<DrawingState> {
        self.drawing.write().expect("bug: Lock was poisoned")
    }

    /// Provides read-only access to the temporary path
    fn temporary_path(&self) -> ReadOnlyRef<Option<Path>> {
        self.temporary_path.read().expect("bug: Lock was poisoned")
    }

    fn set_temporary_path(&mut self, path: Option<Path>) {
        let mut temp = self.temporary_path.write().expect("bug: Lock was poisoned");
        *temp = path;
    }

    /// See [`Turtle::poll_event()`](struct.Turtle.html#method.poll_event).
    pub fn poll_event(&mut self) -> Option<Event> {
        match self.events_channel.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Empty) => None, // Do nothing
            // The window has been closed
            Err(TryRecvError::Disconnected) => self.exit_process(), // Quit
        }
    }

    /// Begin filling the shape drawn by the turtle's movements.
    pub fn begin_fill(&mut self) {
        let fill_color = self.turtle().fill_color;
        self.send_drawing_command(BeginFill(fill_color));
    }

    /// Stop filling the current shape
    pub fn end_fill(&mut self) {
        self.send_drawing_command(EndFill);
    }

    /// Clear the turtle's drawings
    pub fn clear(&mut self) {
        assert!(self.temporary_path().is_none(),
            "bug: The temporary path was still set when the renderer was asked to clear the drawing");
        self.send_drawing_command(Clear);
    }

    /// Move the turtle to the given position without changing its heading.
    pub fn go_to(&mut self, end: Point) {
        let (start, speed, pen) = {
            let turtle = self.turtle();
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
            let turtle = self.turtle();
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

        let TurtleState {heading, speed, ..} = *self.turtle();
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
                let mut turtle = self.turtle_mut();
                animation.advance(&mut *turtle)
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

    #[inline]
    fn send_drawing_command(&mut self, command: DrawingCommand) {
        // During tests, we disable the renderer. That means that if we let this code run, it will
        // quit the application during the tests and make it look like everything passes.
        // We disable this code so that none of that happens.
        #[cfg(not(any(feature = "test", test)))]
        self.drawing_channel.send(command).unwrap_or_else(|_| {
            // The channel is closed which means the window was closed
            // quit immediately
            self.exit_process();
        });
    }

    /// Exits the current process with the correct error code
    ///
    /// Panics if the thread handle has already been consumed
    #[inline]
    fn exit_process(&mut self) -> ! {
        if let Some(handle) = self.thread_handle.take() {
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
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap_or_else(|_| {
                // If this returns an error, the other thread panicked
                process::exit(1);
            });
        }
    }
}
