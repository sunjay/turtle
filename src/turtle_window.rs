use std::thread;
use std::process;
use std::time::Instant;
use std::sync::mpsc::{self, TryRecvError};

use piston_window::{
    PistonWindow,
    WindowSettings,
    math,
    clear,
    line,
    polygon,
};

use radians::{self, Radians, TWO_PI};
use extensions::{ToCanvasCoordinates, AsMillis};
use {Speed, Color, Distance};

pub enum Command {
    /// Moves the turtle in the direction it is facing (based on its heading) by the given amount
    MoveForward {
        distance: Distance,
    },
    /// Rotates the turtle in place (without moving) by the given angle in radians
    Rotate {
        angle: Radians,
        clockwise: bool,
    },
    EnablePen {
        enabled: bool,
    },
}

pub struct Response;

#[derive(Debug, Clone)]
pub struct Pen {
    pub enabled: bool,
    pub thickness: f64,
    pub color: Color,
}

pub struct TurtleState {
    pub position: [f64; 2],
    pub heading: Radians,
    pub speed: Speed,
}

impl Default for TurtleState {
    fn default() -> TurtleState {
        TurtleState {
            position: Default::default(),
            heading: Radians::from_degrees_value(90.),
            speed: Speed::Instant,
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    start: [f64; 2],
    end: [f64; 2],
    pen: Pen,
}

struct DrawingState {
    pub pen: Pen,
    pub background: Color,
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

pub struct TurtleWindow {
    thread_handle: Option<thread::JoinHandle<()>>,
    transmitter: mpsc::Sender<Command>,
    receiver: mpsc::Receiver<Response>,
}

impl TurtleWindow {
    pub fn new() -> TurtleWindow {
        let (drawing_tx, drawing_rx) = mpsc::channel();
        let (main_tx, main_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut window: PistonWindow = WindowSettings::new(
                "Turtle", [800, 600]
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
                background: Color::White,
            };

            while let Some(e) = window.next() {
                match drawing_rx.try_recv() {
                    Ok(command) => {
                        if animation.is_some() {
                            unreachable!("The main thread did not wait for the animation to complete before sending another command")
                        }
                        match command {
                            Command::MoveForward {distance} => {
                                if distance != 0. {
                                    let start = turtle.position;
                                    let x = distance * turtle.heading.cos();
                                    let y = distance * turtle.heading.sin();
                                    let end = math::add(start, [x, y]);
                                    animation = Some(Animation {
                                        kind: AnimationKind::Move {
                                            path: Path {
                                                start, end,
                                                pen: drawing.pen.clone(),
                                            },
                                        },
                                        speed: turtle.speed,
                                        start: Instant::now(),
                                    });
                                }
                            },
                            Command::Rotate {angle, clockwise} => {
                                if angle != radians::ZERO {
                                    let target_angle = turtle.heading + if clockwise {
                                        -angle
                                    }
                                    else {
                                        angle
                                    };
                                    assert!(target_angle != turtle.heading);
                                    animation = Some(Animation {
                                        kind: AnimationKind::Rotation {
                                            target_angle,
                                            clockwise,
                                        },
                                        speed: turtle.speed,
                                        start: Instant::now(),
                                    });
                                }
                            },
                            Command::EnablePen {enabled} => {
                                drawing.pen.enabled = enabled;
                                main_tx.send(Response).unwrap();
                            },
                        }
                    },
                    Err(TryRecvError::Empty) => {}, // Do nothing
                    Err(TryRecvError::Disconnected) => break, // Quit
                }

                window.draw_2d(&e, |c, g| {
                    clear(drawing.background.into(), g);

                    let view = c.get_view_size();
                    let width = view[0] as f64;
                    let height = view[1] as f64;
                    let center = [width * 0.5, height * 0.5];

                    let mut animation_complete = false;
                    if let Some(Animation {ref kind, ref speed, ref start}) = animation {
                        let elapsed = start.elapsed().as_millis() as f64;

                        match *kind {
                            AnimationKind::Move {ref path} => {
                                // This code finds the point on the line between start and
                                // end that we are supposed to be at right now using some
                                // vector math.
                                //
                                // The basic idea is to find the vector that represents the
                                // line between start and end, normalize it into a direction,
                                // and then scale that vector to be the size that it should be
                                // at the elapsed time based on the speed.
                                let &Path {start, end, ref pen} = path;
                                let speed = speed.to_absolute(); // px per second
                                let path_line = math::sub(start, end);
                                let path_length = math::square_len(path_line).sqrt();
                                let direction = math::mul_scalar(path_line, 1./path_length);
                                debug_assert_eq!((math::square_len(direction).sqrt() * 1000.).round(), 1000.);
                                // (px / sec) * (sec / 1000ms) * ms => px
                                let offset = math::mul_scalar(direction, speed / 1000. * elapsed);
                                let offset_length = math::square_len(offset).sqrt();

                                let current = if speed.is_infinite() || offset_length >= path_length {
                                    paths.push(path.clone());
                                    animation_complete = true;
                                    end
                                }
                                else {
                                    math::add(start, offset)
                                };

                                turtle.position = current;

                                let &Pen {thickness, color, enabled} = pen;
                                if enabled {
                                    let start = start.to_canvas_coords(center);
                                    let current = current.to_canvas_coords(center);

                                    line(color.into(), thickness,
                                        [start[0], start[1], current[0], current[1]],
                                        c.transform, g);
                                }
                            },
                            AnimationKind::Rotation {target_angle, clockwise} => {
                                let speed = speed.to_rotation();
                                let target_angle = target_angle % TWO_PI;

                                let heading = turtle.heading;
                                let current = (heading + speed / 1000. * elapsed) % TWO_PI;
                                let current = if speed.is_infinite() || current > target_angle {
                                    animation_complete = true;
                                    target_angle
                                }
                                else {
                                    let current = if clockwise {
                                        TWO_PI - current
                                    } else { current };

                                    current
                                };
                                assert!(!current.is_nan(), "bug: heading became NaN");
                                turtle.heading = current;
                            },
                        }
                    }
                    if animation_complete {
                        animation = None;
                        main_tx.send(Response).unwrap();
                    }

                    for path in &paths {
                        if !path.pen.enabled {
                            continue;
                        }

                        let Path {start, end, ref pen} = *path;
                        let start = start.to_canvas_coords(center);
                        let end = end.to_canvas_coords(center);

                        line(pen.color.into(), pen.thickness,
                            [start[0], start[1], end[0], end[1]],
                            c.transform, g);
                    }

                    // Draw the turtle's shell
                    let cos = turtle.heading.cos();
                    let sin = turtle.heading.sin();
                    let turtle_x = turtle.position[0];
                    let turtle_y = turtle.position[1];
                    let shell: Vec<_> = [
                        [0., 15.],
                        [10., 0.],
                        [0., -15.],
                    ].into_iter().map(|pt| {
                        // Rotate each point by the heading and add the current turtle position
                        let x = cos * pt[0] - sin * pt[1] + turtle_x;
                        let y = sin * pt[0] + cos * pt[1] + turtle_y;
                        [x, y].to_canvas_coords(center)
                    }).collect();

                    polygon(Color::White.into(), &shell, c.transform, g);
                    for i in 0..shell.len() {
                        let start = shell[i];
                        let end = shell[(i + 1) % shell.len()];

                        line(Color::Black.into(), 1.,
                            [start[0], start[1], end[0], end[1]],
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

impl Drop for TurtleWindow {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
