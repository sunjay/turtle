// During tests, we disable the renderer and that causes a bunch of warnings
// See Cargo.toml for an explanation of this attribute
#![cfg_attr(any(feature = "test", test), allow(dead_code, unused_variables, unused_imports))]

use std::thread;
use std::process;
use std::time::Instant;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use piston_window::math;

use renderer::{Renderer, DrawingCommand};
use animation::{Animation, MoveAnimation, RotateAnimation, AnimationStatus};
use state::{TurtleState, DrawingState, Path, Pen};
use radians::{self, Radians};
use {Speed, Distance, Event, color};

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
        let (drawing_tx, drawing_rx) = mpsc::channel();
        let (events_tx, events_rx) = mpsc::channel();

        let mut turtle_window = Self {
            thread_handle: None,
            drawing_channel: drawing_tx,
            events_channel: events_rx,
            turtle: Arc::new(RwLock::new(TurtleState {
                position: [0., 0.],
                heading: Radians::from_degrees_value(90.),
                speed: Speed::Five,
                visible: true,
            })),
            drawing: Arc::new(RwLock::new(DrawingState {
                pen: Pen {
                    enabled: true,
                    thickness: 1.,
                    color: color::BLACK,
                },
                fill_color: color::BLACK,
                background: color::WHITE,
            })),
            temporary_path: Arc::new(RwLock::new(None)),
        };

        let read_only = turtle_window.read_only();
        let handle = thread::spawn(move || {
            // See Cargo.toml for an explanation of this attribute
            #[cfg(not(any(feature = "test", test)))]
            Renderer::new().run(drawing_rx, events_tx, read_only);
        });

        turtle_window.thread_handle = Some(handle);
        turtle_window
    }

    /// Provide a read-only version of the state
    pub fn read_only(&self) -> ReadOnly {
        ReadOnly {
            turtle: self.turtle.clone(),
            drawing: self.drawing.clone(),
            temporary_path: self.temporary_path.clone(),
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
            Err(TryRecvError::Disconnected) => process::exit(0), // Quit
        }
    }

    /// Begin filling the shape drawn by the turtle's movements.
    pub fn begin_fill(&mut self) {
        let fill_color = self.drawing().fill_color;
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

    /// Move the turtle forward by the given distance. To move backwards, use a negative distance.
    ///
    /// The turtle's motion will be animated based on the speed
    pub fn forward(&mut self, distance: Distance) {
        if distance == 0. {
            return;
        }

        let TurtleState {position: start, heading, speed, ..} = *self.turtle();
        let x = distance * heading.cos();
        let y = distance * heading.sin();
        let end = math::add(start, [x, y]);

        let speed = speed.to_absolute(); // px per second
        let total_millis = (distance / speed * 1000.).abs();

        let animation = MoveAnimation {
            path: Path {
                start,
                end,
                pen: self.drawing().pen.clone(),
            },
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
        let total_millis = total_millis.to_radians();

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
        self.drawing_channel.send(command).unwrap_or_else(|_| {
            // The channel is closed which means the window was closed
            // quit immediately
            process::exit(0);
        });
    }
}

impl Drop for TurtleWindow {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
