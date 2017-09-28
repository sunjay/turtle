use std::time::Instant;
use std::sync::mpsc::{self, TryRecvError};

use piston_window::{
    PistonWindow,
    G2d,
    context,
    math,
    clear,
    line,
    polygon,
};

use radians::{self, Radians};
use animation::{Animation, AnimationStatus};
use extensions::ToCanvasCoordinates;
use {Point, Speed, Color, Distance};

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
}

pub struct Response;

#[derive(Debug, Clone)]
pub struct Pen {
    pub enabled: bool,
    pub thickness: f64,
    pub color: Color,
}

pub struct TurtleState {
    pub position: Point,
    pub heading: Radians,
    pub speed: Speed,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub start: Point,
    pub end: Point,
    pub pen: Pen,
}

struct DrawingState {
    pub pen: Pen,
    pub background: Color,
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
                speed: Speed::Eight,
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
                Ok(command) => self.handle_command(command),
                Err(TryRecvError::Empty) => {}, // Do nothing
                Err(TryRecvError::Disconnected) => break, // Quit
            }

            window.draw_2d(&e, |c, g| {
                clear(self.drawing.background.into(), g);

                let view = c.get_view_size();
                let width = view[0] as f64;
                let height = view[1] as f64;
                let center = [width * 0.5, height * 0.5];

                let animation_complete = self.animate(c, g, center);
                if animation_complete {
                    self.animation = None;
                    main_tx.send(Response).unwrap();
                }

                for path in &self.paths {
                    self.render_path(c, g, center, path);
                }

                self.render_shell(c, g, center);
            });
        }
    }

    fn handle_command(&mut self, command: Command) {
        if self.animation.is_some() {
            unreachable!("The main thread did not wait for the animation to complete before sending another command")
        }

        match command {
            Command::MoveForward {distance} => {
                if distance == 0. {
                    return;
                }

                let start = self.turtle.position;
                let x = distance * self.turtle.heading.cos();
                let y = distance * self.turtle.heading.sin();
                let end = math::add(start, [x, y]);

                let speed = self.turtle.speed;
                let speed = speed.to_absolute(); // px per second
                let total_millis = distance / speed * 1000.;

                self.animation = Some(Animation::Move {
                    path: Path {
                        start,
                        end,
                        pen: self.drawing.pen.clone(),
                    },
                    timer: Instant::now(),
                    total_millis,
                });
            },
            Command::Rotate {angle, clockwise} => {
                if angle == radians::ZERO {
                    return;
                }

                let speed = self.turtle.speed;
                let speed = speed.to_rotation(); // radians per second
                let total_millis = angle / speed * 1000.;
                let total_millis = total_millis.value();

                self.animation = Some(Animation::Rotate {
                    start: self.turtle.heading,
                    delta_angle: angle,
                    clockwise,
                    timer: Instant::now(),
                    total_millis,
                });
            },
        }
    }

    /// Moves to the next step of the animation and returns true if the animation is complete.
    fn animate(&mut self, c: context::Context, g: &mut G2d, center: Point) -> bool {
        if let Some(ref animation) = self.animation {
            match animation.advance(&mut self.turtle) {
                AnimationStatus::Running(path) => {
                    match path {
                        Some(ref path) if path.pen.enabled => self.render_path(c, g, center, &path),
                        _ => {},
                    }

                    false
                },
                AnimationStatus::Complete(path) => {
                    match path {
                        Some(path) =>  if path.pen.enabled {
                            self.paths.push(path)
                        },
                        _ => {},
                    }

                    true
                },
            }
        }
        else {
            true
        }
    }

    /// Render a path assuming that its pen is enabled
    fn render_path(&self, c: context::Context, g: &mut G2d, center: Point, path: &Path) {
        let &Path {start, end, ref pen} = path;
        let &Pen {thickness, color, enabled} = pen;
        debug_assert!(enabled, "bug: attempt to render path when pen was not enabled");

        let start = start.to_canvas_coords(center);
        let end = end.to_canvas_coords(center);

        line(color.into(), thickness,
            [start[0], start[1], end[0], end[1]],
            c.transform, g);
    }

    /// Draw the turtle's shell
    fn render_shell(&self, c: context::Context, g: &mut G2d, center: Point) {
        // Calculate all the points on the shell by rotating the shell shape by the turtle's
        // heading and moving it to the turtle's position
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

        // Draw the turtle shell with its background first, then its border
        polygon(Color::White.into(), &shell, c.transform, g);
        for i in 0..shell.len() {
            let start = shell[i];
            let end = shell[(i + 1) % shell.len()];

            line(Color::Black.into(), 1.,
            [start[0], start[1], end[0], end[1]],
            c.transform, g);
        }
    }
}
