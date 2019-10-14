#[cfg(target_arch = "wasm32")]
compile_error!("This module should not be included when compiling to wasm");

use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::path;

use piston_window::{clear, context, line, polygon, AdvancedWindow, Event as PistonEvent, G2d,
    Input, PistonWindow, WindowSettings};

use svg;
use svg::node::element::{Line as SvgLine, Polygon as SvgPolygon, Rectangle as SvgRect};

use crate::app::TurtleApp;
use crate::event::from_piston_event;
use crate::extensions::ConvertScreenCoordinates;
use crate::query::DrawingCommand;
use crate::state::{DrawingState, Path, Pen, Polygon, TurtleState};
use crate::{color, Color, Event, Point};

fn update_window(window: &mut PistonWindow, current: DrawingState, next: DrawingState) -> DrawingState {
    if next.title != current.title {
        window.set_title(next.title.clone());
    }
    if next.width != current.width || next.height != current.height {
        window.set_size((next.width, next.height));
    }
    if next.maximized != current.maximized {
        window.window.window.set_maximized(next.maximized);
    }
    if next.fullscreen != current.fullscreen {
        if next.fullscreen {
            window
                .window
                .window
                .set_fullscreen(Some(window.window.window.get_current_monitor()));
        } else {
            window.window.window.set_fullscreen(None);
        }
    }
    next
}

#[derive(Debug)]
pub enum Drawing {
    Path(Path),
    Polygon(Polygon),
}

pub struct Renderer {
    app: TurtleApp,
    drawings: Vec<Drawing>,
    /// Polygon that is currently in the process of being filled
    /// Removed when EndFill is sent
    fill_polygon: Option<(Vec<Path>, Polygon)>,
}

impl Renderer {
    pub fn new(app: TurtleApp) -> Renderer {
        Self {
            app,
            drawings: Vec::new(),
            fill_polygon: None,
        }
    }

    pub fn run(&mut self, drawing_rx: mpsc::Receiver<DrawingCommand>, events_tx: mpsc::Sender<Event>) {
        let state = self.app.read_only();

        // This check isn't foolproof. Someone can always create a thread named "main".
        if thread::current().name().unwrap_or("") != "main" {
            // In order to maintain compatibility with MacOS, we need to make sure that windows are
            // only created on the main thread. We do this check on all platforms so that no one
            // can accidentally make a change that creates the window off of the main thread.
            unreachable!("bug: windows can only be created on the main thread");
        }

        let mut window: PistonWindow = WindowSettings::new(
            &*state.drawing().title,
            (state.drawing().width, state.drawing().height)
        ).exit_on_esc(true).build().expect("bug: could not build window");

        // We keep a copy of the DrawingState so that we can tell when it is updated and we need
        // to change something on the window
        let mut current_drawing = DrawingState::default();

        let mut center = state.drawing().center;

        'renderloop: while let Some(event) = window.next() {
            match event {
                PistonEvent::Input(Input::Resize(width, height)) => {
                    let width = width as u32;
                    let height = height as u32;
                    if width != current_drawing.width || height != current_drawing.height {
                        let mut drawing = self.app.drawing_mut();
                        drawing.width = width;
                        drawing.height = height;
                    }
                }
                _ => {}
            }

            if let Some(event) = from_piston_event(&event, |pt| pt.to_local_coords(center)) {
                match events_tx.send(event) {
                    Ok(_) => {}
                    // Quit - the server thread must have quit
                    Err(_) => break,
                }
            }

            // Need to handle all of the queries we receive at once so that any lag caused by
            // how long rendering takes doesn't cause any problems
            loop {
                match drawing_rx.try_recv() {
                    Ok(cmd) => self.handle_drawing_command(cmd),
                    Err(TryRecvError::Empty) => break,                    // Do nothing
                    Err(TryRecvError::Disconnected) => break 'renderloop, // Quit
                }
            }

            // Update the window based on any changes in the DrawingState
            current_drawing = update_window(&mut window, current_drawing, state.drawing().clone());

            window.draw_2d(&event, |c, g| {
                let view = c.get_view_size();
                let width = view[0] as f64;
                let height = view[1] as f64;
                center = state.drawing().center.to_screen_coords(Point {
                    x: width * 0.5,
                    y: height * 0.5,
                });

                // We clone the relevant state before rendering so that the rendering thread
                // doesn't need to keep locking, waiting or making the main thread wait
                let drawing = state.drawing().clone();
                let temporary_path = state.temporary_path().clone();
                let turtle = state.turtle().clone();

                self.render(c, g, center, &drawing, &temporary_path, &turtle);
            });
        }
    }

    /// Handles a drawing command sent from the main thread
    fn handle_drawing_command(&mut self, command: DrawingCommand) {
        //NOTE: Do not pass the ReadOnly state to this function. By the time a DrawingCommand is
        // handled, that state may be completely out of date
        use DrawingCommand::*;
        match command {
            StorePath(path) => {
                if self.fill_polygon.is_some() {
                    let &mut (ref mut border, ref mut poly) = self.fill_polygon.as_mut().unwrap();
                    border.push(path.clone());

                    let Path { start, end, .. } = path;
                    if poly.vertices.last().map_or(true, |&v| v != start) {
                        poly.vertices.push(start);
                    }
                    poly.vertices.push(end);
                } else if path.pen.enabled {
                    self.drawings.push(Drawing::Path(path));
                }
            },
            BeginFill(fill_color) => {
                // Calling begin_fill multiple times is okay, it just won't do anything until
                // end_fill is called
                self.fill_polygon = self.fill_polygon.take().or_else(|| {
                    Some((
                        Vec::new(),
                        Polygon {
                            vertices: Vec::new(),
                            fill_color,
                        },
                    ))
                });
            },
            // Calling end_fill multiple times is not a problem
            EndFill => if let Some((border, poly)) = self.fill_polygon.take() {
                // Always add the border over the filled polygon so the border is drawn on top
                self.drawings.push(Drawing::Polygon(poly));
                self.drawings.extend(
                    border
                        .into_iter()
                        .filter_map(|p| if p.pen.enabled { Some(Drawing::Path(p)) } else { None }),
                );
            },
            Clear => {
                self.drawings.clear();
                // Because clear doesn't actually shorten the capacity of the vector
                // We don't want to leak memory by leaving this Vec at whatever its size was before
                self.drawings.shrink_to_fit();
                self.fill_polygon.take();
            },
            SaveSVG(path_buf) => self.save_svg(path_buf),
        }
    }

    /// The main rendering route. Dispatches to other functions as needed.
    fn render(
        &self,
        c: context::Context,
        g: &mut G2d<'_>,
        center: Point,
        drawing: &DrawingState,
        temporary_path: &Option<Path>,
        turtle: &TurtleState,
    ) {
        let background = drawing.background;
        clear(background.into(), g);

        for drawing in &self.drawings {
            match *drawing {
                Drawing::Path(ref path) => self.render_path(c, g, center, path),
                Drawing::Polygon(ref poly) => self.render_polygon(c, g, center, poly.fill_color, poly.vertices.iter()),
            }
        }

        if let Some(&(ref border, ref poly)) = self.fill_polygon.as_ref() {
            // If the temporary_path is not None, we need to add it to the polygon being
            // filled or else the polygon will fall one edge behind in the animation
            let extra = temporary_path.as_ref().map_or(Vec::new(), |&Path { start, end, .. }| {
                if poly.vertices.last().map_or(true, |&v| v != start) {
                    vec![start, end]
                } else {
                    vec![end]
                }
            });
            self.render_polygon(c, g, center, poly.fill_color, poly.vertices.iter().chain(extra.iter()));

            for path in border {
                if path.pen.enabled {
                    self.render_path(c, g, center, path);
                }
            }
        }

        if let Some(ref path) = *temporary_path {
            if path.pen.enabled {
                self.render_path(c, g, center, path);
            }
        }

        self.render_shell(c, g, center, turtle);
    }

    /// Render a path assuming that its pen is enabled
    fn render_path(&self, c: context::Context, g: &mut G2d<'_>, center: Point, path: &Path) {
        let &Path { start, end, ref pen } = path;
        let &Pen { thickness, color, enabled } = pen;
        debug_assert!(enabled, "bug: attempt to render path when pen was not enabled");

        let start = start.to_screen_coords(center);
        let end = end.to_screen_coords(center);

        line(color.into(), thickness, [start.x, start.y, end.x, end.y], c.transform, g);
    }

    /// Render a polygon given its vertices
    fn render_polygon<'a, T: Iterator<Item = &'a Point>>(
        &self,
        c: context::Context,
        g: &mut G2d<'_>,
        center: Point,
        fill_color: Color,
        verts: T,
    ) {
        // Performance note: Why make this function generic instead of just taking Polygon?
        // Answer: render_polygon is called multiple times. It's called repeatedly in a loop, and
        // it is also called when drawing the current fill_polygon. Rather than moving the
        // code to *maybe* add the temporary_path to the rendered polygon into this method, we
        // avoid branching unnecessarily by allowing the repeated caller to do what is fast, then
        // doing the slow thing (checking if the path is None) only once when it is needed.
        //
        // We pass in the points as an Iterator so that they do not need to be collected into any
        // struct. This avoids an allocation that really isn't needed since these are all temporary
        // anyway. Everything is going to get copied anyway on the next line. No need to do it
        // twice.
        //
        // See the commit before this comment was added for the approach that would have required
        // branching in every iteration of the loop where render_polygon is called over and over
        // again.
        let verts = verts.map(|p| p.to_screen_coords(center).into()).collect::<Vec<_>>();
        polygon(fill_color.into(), &verts, c.transform, g);
    }

    /// Draw the turtle's shell
    fn render_shell(
        &self,
        c: context::Context,
        g: &mut G2d<'_>,
        center: Point,
        &TurtleState {
            position,
            heading,
            visible,
            ..
        }: &TurtleState,
    ) {
        // Calculate all the points on the shell by rotating the shell shape by the turtle's
        // heading and moving it to the turtle's position
        if !visible {
            return;
        }

        let cos = heading.cos();
        let sin = heading.sin();
        let turtle_x = position.x;
        let turtle_y = position.y;
        let shell: Vec<_> = [[0., 15.], [10., 0.], [0., -15.]]
            .into_iter()
            .map(|pt| {
                // Rotate each point by the heading and add the current turtle position
                let x = cos * pt[0] - sin * pt[1] + turtle_x;
                let y = sin * pt[0] + cos * pt[1] + turtle_y;
                Point { x, y }.to_screen_coords(center).into()
            })
            .collect();

        // Draw the turtle shell with its background first, then its border
        polygon(color::WHITE.into(), &shell, c.transform, g);
        for i in 0..shell.len() {
            let start = shell[i];
            let end = shell[(i + 1) % shell.len()];

            line(color::BLACK.into(), 1., [start[0], start[1], end[0], end[1]], c.transform, g);
        }
    }

    /// Save the drawings to SVG
    fn save_svg(&mut self, path: &path::Path) {
        // Returns rgba string of Color in CSS functional syntax
        fn rgba_string(color: Color) -> String {
            format!("rgba({}, {}, {}, {})", color.red as u8, color.green as u8, color.blue as u8, color.alpha)
        }

        let drawing_state = self.app.drawing();
        let mut document = svg::Document::new()
            .set("viewBox", (0, 0, drawing_state.width, drawing_state.height));

        // set background color - https://stackoverflow.com/a/11293812/9276882
        let bg_color = drawing_state.background;
        let background = SvgRect::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", rgba_string(bg_color));

        document = document.add(background);

        let center = drawing_state.center.to_screen_coords(Point {
            x: drawing_state.width as f64 * 0.5,
            y: drawing_state.height as f64 * 0.5,
        });

        for drawing in &self.drawings {
            match *drawing {
                Drawing::Path(ref path) => {
                    let &Path { start, end, ref pen } = path;
                    let &Pen { thickness, color, enabled: _ } = pen;

                    let start = start.to_screen_coords(center);
                    let end = end.to_screen_coords(center);

                    let line = SvgLine::new()
                        .set("x1", start.x)
                        .set("y1", start.y)
                        .set("x2", end.x)
                        .set("y2", end.y)
                        .set("stroke", rgba_string(*color))
                        .set("stroke-width", format!("{}px", thickness * 2.0));

                    // Note that in above code `stroke-width` is twice the `thickness`.
                    // As per https://docs.piston.rs/piston_window/piston_window/fn.line.html,
                    // second param of piston_window::line is radius.
                    // So radius (thickness) = 1 takes 2 pixels on screen.
                    // Hence, stroke-width of svg element is double the thickness.

                    document = document.add(line);
                },
                Drawing::Polygon(ref poly) => {
                    let points = poly.vertices.iter()
                        .map(|p| {
                            let point = p.to_screen_coords(center);
                            format!("{},{} ", point.x, point.y)
                        })
                        .fold("".to_string(), |acc, x| acc + &x);

                    let color = poly.fill_color;
                    let polygon = SvgPolygon::new()
                        .set("fill", rgba_string(poly.fill_color))
                        .set("points", points);

                    document = document.add(polygon);
                },
            }
        }

        let export_to_svg = svg::save(path, &document);
        match export_to_svg {
            Ok(()) => (),
            Err(error) => eprintln!("Error saving SVG file : {:#?}", error),
        }
    }
}
