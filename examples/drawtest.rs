#![allow(unused_variables, unused_mut, dead_code)]

extern crate piston_window;

extern crate turtleide;

use canvas::{TurtleCanvas, Command, Radians};

fn main() {
    let canvas = TurtleCanvas::new();

    for i in 0..361 {
        println!("1");
        canvas.apply(Command::Move {
            distance: 10.,
        });
        println!("2");
        canvas.apply(Command::Rotate {
            angle: Radians(1f64.to_radians()),
            clockwise: true,
        });
        println!("3");
        //canvas.apply(Command::Pen {
        //    enabled: i % 2 == 0,
        //});
    }
}

mod canvas {
    use std::thread;
    use std::process;
    use std::time::{Instant, Duration};
    use std::sync::mpsc::{self, TryRecvError};
    use std::ops::{Add, Mul};

    use piston_window::*;

    use turtleide::Speed;

    #[derive(Default, Clone, Copy, Debug, PartialOrd, PartialEq)]
    pub struct Radians(pub f64);

    impl Radians {
        pub fn cos(self) -> f64 {
            self.0.cos()
        }

        pub fn sin(self) -> f64 {
            self.0.sin()
        }
    }

    impl Add for Radians {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Radians(self.0 + other.0)
        }
    }

    impl Mul<f64> for Radians {
        type Output = Self;

        fn mul(self, other: f64) -> Self {
            Radians(self.0 * other)
        }
    }

    trait LinearInterpolation {
        /// Interpolate between self and the given target
        ///
        /// t must be between 0.0 and 1.0
        fn interpolate(self, target: Self, t: f64) -> Self;
    }

    impl LinearInterpolation for f64 {
        fn interpolate(self, target: Self, t: f64) -> Self {
            (target - self) * t
        }
    }

    impl LinearInterpolation for (f64, f64) {
        fn interpolate(self, target: Self, t: f64) -> Self {
            let (x1, y1) = self;
            let (x2, y2) = target;

            let x = x1.interpolate(x2, t);
            let y = y1 + (x - x1) * (y2 - y1) / (x2 - x1);
            (x, y)
        }
    }

    pub enum Command {
        Move {
            distance: f64,
        },
        Rotate {
            angle: Radians,
            clockwise: bool,
        },
        Pen {
            enabled: bool,
        },
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Color {
        Black,
        Transparent,
    }

    impl From<Color> for types::Color {
        fn from(color: Color) -> Self {
            match color {
                Color::Black => [0., 0., 0., 255.],
                Color::Transparent => [0., 0., 0., 0.],
            }
        }
    }

    #[derive(Debug, Clone)]
    struct Path {
        start: (f64, f64),
        end: (f64, f64),
        pen: Pen,
    }

    impl Path {
        /// Returns the size of the line represented by this path
        fn len(&self) -> f64 {
            let (x1, y1) = self.start;
            let (x2, y2) = self.start;
            ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
        }

        /// Linearly interpolate between the points in this path
        fn interpolate(&self, t: f64) -> (f64, f64) {
            self.start.interpolate(self.end, t)
        }
    }

    type Response = Result<(), ()>;

    struct TurtleState {
        pub position: (f64, f64),
        pub heading: Radians,
        pub speed: Speed,
    }

    impl Default for TurtleState {
        fn default() -> TurtleState {
            TurtleState {
                position: Default::default(),
                heading: Radians(90f64.to_radians()),
                speed: Default::default(),
            }
        }
    }

    struct DrawingState {
        pen: Pen,
        //pub background: Color,
    }

    #[derive(Debug, Clone)]
    struct Pen {
        pub enabled: bool,
        pub thickness: f64,
        pub color: Color,
    }

    struct Animation {
        kind: AnimationKind,
        speed: Speed,
        start: Instant,
    }

    enum AnimationKind {
        Move {
            path: Path,
        },
        Rotation {
            target_angle: Radians,
            clockwise: bool,
        },
    }

    pub struct TurtleCanvas {
        thread_handle: Option<thread::JoinHandle<()>>,
        transmitter: mpsc::Sender<Command>,
        receiver: mpsc::Receiver<Response>,
    }

    impl TurtleCanvas {
        pub fn new() -> TurtleCanvas {
            let (drawing_tx, drawing_rx) = mpsc::channel();
            let (main_tx, main_rx) = mpsc::channel();

            let handle = thread::spawn(move || {
                let mut window: PistonWindow = WindowSettings::new(
                    "Turtle IDE", [800, 600]
                ).exit_on_esc(true).build().unwrap();
                let mut paths: Vec<Path> = Vec::new();
                let mut animation: Option<Animation> = None;
                let mut turtle = TurtleState::default();
                let mut drawing = DrawingState {
                    pen: Pen {
                        enabled: true,
                        thickness: 1.,
                        color: Color::Black,
                    },
                };
                let center = (200., 200.);

                while let Some(e) = window.next() {
                    match drawing_rx.try_recv() {
                        Ok(command) => {
                            match command {
                                Command::Move {distance} => {
                                    if animation.is_some() {
                                        unreachable!("The main thread did not wait for the animation to complete before sending another command")
                                    }
                                    if distance != 0. {
                                        let current_x = center.0 + turtle.position.0;
                                        let current_y = center.1 + turtle.position.1;
                                        let x = distance * turtle.heading.cos();
                                        let y = distance * turtle.heading.sin();
                                        println!("move forward {:?}", (x, y));
                                        animation = Some(Animation {
                                            kind: AnimationKind::Move {
                                                path: Path {
                                                    start: (current_x, current_y),
                                                    end: (current_x + x, current_y + y),
                                                    pen: drawing.pen.clone(),
                                                },
                                            },
                                            speed: turtle.speed,
                                            start: Instant::now(),
                                        });
                                    }
                                },
                                Command::Rotate {angle, clockwise} => {
                                    if animation.is_some() {
                                        unreachable!("The main thread did not wait for the animation to complete before sending another command")
                                    }
                                    animation = Some(Animation {
                                        kind: AnimationKind::Rotation {
                                            target_angle: turtle.heading + angle,
                                            clockwise,
                                        },
                                        speed: turtle.speed,
                                        start: Instant::now(),
                                    });
                                },
                                Command::Pen {enabled} => unimplemented!(),
                            }
                        },
                        Err(TryRecvError::Empty) => {}, // Do nothing
                        Err(TryRecvError::Disconnected) => break, // Quit
                    }

                    window.draw_2d(&e, |c, g| {
                        clear([1.0; 4], g);

                        let mut animation_complete = false;
                        if let Some(Animation {ref kind, ref speed, ref start}) = animation {
                            let elapsed = start.elapsed();

                            match *kind {
                                AnimationKind::Move {ref path} => {
                                    let speed = speed.to_absolute();
                                    let length = path.len();
                                    let total_time = Duration::from_millis((length * 1000. / speed) as u64);

                                    let progress = elapsed.as_secs() as f64 / total_time.as_secs() as f64;
                                    if progress > 1.0 {
                                        paths.push(path.clone());
                                        animation_complete = true;
                                    }
                                    else {
                                        println!("progress = {}", progress);
                                        let current = path.interpolate(progress);
                                        turtle.position = current;
                                        println!("turtle.position = {:?}", turtle.position);

                                        let Pen {thickness, color, ..} = path.pen;
                                        let start = path.start;
                                        line(color.into(), thickness,
                                        [start.0, start.1, current.0, current.1],
                                        c.transform, g);
                                    }
                                },
                                AnimationKind::Rotation {target_angle, clockwise} => {
                                    // TODO: Use the returned value in Radians and impl Div for Radians
                                    let speed = speed.to_rotation();
                                    animation_complete = true;
                                },
                            }
                        }
                        if animation_complete {
                            animation = None;
                            main_tx.send(Ok(())).unwrap();
                        }

                        for path in &paths {
                            if !path.pen.enabled {
                                continue;
                            }

                            let Path {start, end, ref pen} = *path;
                            line(pen.color.into(), pen.thickness,
                                [start.0, start.1, end.0, end.1],
                                c.transform, g);
                        }
                    });
                }
            });

            Self {
                thread_handle: Some(handle),
                transmitter: drawing_tx,
                receiver: main_rx,
            }
        }

        pub fn apply(&self, command: Command) {
            let result = self.transmitter.send(command).map_err(|_| ());
            let result = result.and_then(|_| {
                // Wait for the drawing animation to complete
                self.receiver.recv().map_err(|_| ())
            });
            match result {
                Ok(_) => {},
                Err(_) => {
                    // The connection has been closed so the window was closed
                    // or an error occurred on that thread
                    process::exit(0);
                },
            }
        }
    }

    impl Drop for TurtleCanvas {
        fn drop(&mut self) {
            if let Some(handle) = self.thread_handle.take() {
                handle.join().unwrap();
            }
        }
    }
}
