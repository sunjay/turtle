use std::thread;
use std::process;
use std::time::Instant;
use std::sync::{mpsc, Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use fps_clock::FpsClock;
use piston_window::{
    PistonWindow,
    WindowSettings,
    math,
};

use renderer::Renderer;
use animation::{Animation, MoveAnimation, RotateAnimation, AnimationStatus};
use state::{TurtleState, DrawingState, Path, Pen};
use radians::{self, Radians};
use {Speed, Color, Distance};

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
    /// Channel for sending completed paths so they can be stored and re-rendered
    /// in the rendering thread
    paths_channel: mpsc::Sender<Path>,

    turtle: Shared<TurtleState>,
    drawing: Shared<DrawingState>,
    /// A temporary path for use during animations
    temporary_path: Shared<Option<Path>>,
}

impl TurtleWindow {
    pub fn new() -> TurtleWindow {
        let (paths_tx, paths_rx) = mpsc::channel();

        let mut turtle_window = Self {
            thread_handle: None,
            paths_channel: paths_tx,
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
                    color: Color::Black,
                },
                background: Color::White,
            })),
            temporary_path: Arc::new(RwLock::new(None)),
        };

        let read_only = turtle_window.read_only();
        let handle = thread::spawn(move || {
            let mut window: PistonWindow = WindowSettings::new(
                "Turtle", [800, 600]
            ).exit_on_esc(true).build().unwrap();

            Renderer::new().run(&mut window, paths_rx, read_only);
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
    pub fn turtle_mut(&self) -> MutableRef<TurtleState> {
        self.turtle.write().expect("bug: Lock was poisoned")
    }

    /// Provides read-only access to the drawing
    pub fn drawing(&self) -> ReadOnlyRef<DrawingState> {
        self.drawing.read().expect("bug: Lock was poisoned")
    }

    /// Provides mutable access to the drawing
    pub fn drawing_mut(&self) -> MutableRef<DrawingState> {
        self.drawing.write().expect("bug: Lock was poisoned")
    }

    fn set_temporary_path(&self, path: Option<Path>) {
        let mut temp = self.temporary_path.write().expect("bug: Lock was poisoned");
        *temp = path;
    }

    /// Move the turtle forward by the given distance. To move backwards, use a negative distance.
    ///
    /// The turtle's motion will be animated based on the speed
    pub fn forward(&self, distance: Distance) {
        if distance == 0. {
            return;
        }

        let TurtleState {position: start, heading, speed, ..} = *self.turtle();
        let x = distance * heading.cos();
        let y = distance * heading.sin();
        let end = math::add(start, [x, y]);

        let speed = speed.to_absolute(); // px per second
        let total_millis = distance / speed * 1000.;

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
    pub fn rotate(&self, angle: Radians, clockwise: bool) {
        if angle == radians::ZERO {
            return;
        }

        let TurtleState {heading, speed, ..} = *self.turtle();
        let speed = speed.to_rotation(); // radians per second
        let total_millis = angle / speed * 1000.;
        let total_millis = total_millis.value();

        let animation = RotateAnimation {
            start: heading,
            delta_angle: angle,
            clockwise,
            timer: Instant::now(),
            total_millis,
        };

        self.play_animation(animation);
    }

    fn play_animation<A: Animation>(&self, animation: A) {
        let mut fps = FpsClock::new(60);
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
                        match self.paths_channel.send(path) {
                            Ok(_) => {},
                            /// The channel is closed which means the window was closed
                            Err(_) => {
                                // quit immediately
                                process::exit(0);
                            },
                        };
                    }

                    break;
                },
            }

            fps.tick();
        }
    }
}

impl Drop for TurtleWindow {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
