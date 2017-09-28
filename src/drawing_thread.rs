use std::time::Instant;
use std::sync::mpsc::{self, TryRecvError};

use piston_window::{
    PistonWindow,
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

pub struct DrawingThread {
    paths: Vec<Path>,
    animation: Option<Animation>,
    turtle: TurtleState,
    drawing: DrawingState,
}

impl DrawingThread {
    pub fn new() -> DrawingThread {
        Self {
            paths: Vec::new(),
            animation: None,
            turtle: TurtleState {
                position: [0., 0.],
                heading: Radians::from_degrees_value(90.),
                speed: Speed::Instant,
            },
            drawing: DrawingState {
                pen: Pen {
                    enabled: true,
                    thickness: 1.,
                    color: Color::Black,
                },
                background: Color::White,
            },
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, drawing_rx: mpsc::Receiver<Command>, main_tx: mpsc::Sender<Response>) {
        while let Some(e) = window.next() {
            match drawing_rx.try_recv() {
                Ok(command) => {
                    if self.animation.is_some() {
                        unreachable!("The main thread did not wait for the animation to complete before sending another command")
                    }
                    match command {
                        Command::MoveForward {distance} => {
                            if distance != 0. {
                                let start = self.turtle.position;
                                let x = distance * self.turtle.heading.cos();
                                let y = distance * self.turtle.heading.sin();
                                let end = math::add(start, [x, y]);
                                self.animation = Some(Animation {
                                    kind: AnimationKind::Move {
                                        path: Path {
                                            start, end,
                                            pen: self.drawing.pen.clone(),
                                        },
                                    },
                                    speed: self.turtle.speed,
                                    start: Instant::now(),
                                });
                            }
                        },
                        Command::Rotate {angle, clockwise} => {
                            if angle != radians::ZERO {
                                let target_angle = self.turtle.heading + if clockwise {
                                    -angle
                                }
                                else {
                                    angle
                                };
                                assert!(target_angle != self.turtle.heading);
                                self.animation = Some(Animation {
                                    kind: AnimationKind::Rotation {
                                        target_angle,
                                        clockwise,
                                    },
                                    speed: self.turtle.speed,
                                    start: Instant::now(),
                                });
                            }
                        },
                        Command::EnablePen {enabled} => {
                            self.drawing.pen.enabled = enabled;
                            main_tx.send(Response).unwrap();
                        },
                    }
                },
                Err(TryRecvError::Empty) => {}, // Do nothing
                Err(TryRecvError::Disconnected) => break, // Quit
            }

            window.draw_2d(&e, |c, g| {
                clear(self.drawing.background.into(), g);

                let view = c.get_view_size();
                let width = view[0] as f64;
                let height = view[1] as f64;
                let center = [width * 0.5, height * 0.5];

                let mut animation_complete = false;
                if let Some(Animation {ref kind, ref speed, ref start}) = self.animation {
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
                                self.paths.push(path.clone());
                                animation_complete = true;
                                end
                            }
                            else {
                                math::add(start, offset)
                            };

                            self.turtle.position = current;

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

                            let heading = self.turtle.heading;
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
                            self.turtle.heading = current;
                        },
                    }
                }
                if animation_complete {
                    self.animation = None;
                    main_tx.send(Response).unwrap();
                }

                for path in &self.paths {
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
                let cos = self.turtle.heading.cos();
                let sin = self.turtle.heading.sin();
                let turtle_x = self.turtle.position[0];
                let turtle_y = self.turtle.position[1];
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
    }
}
