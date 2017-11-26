// During tests, we disable the renderer and that causes a bunch of warnings that we just want
// to get rid of.
// See Cargo.toml for an explanation of this attribute
#![cfg_attr(any(feature = "test", test), allow(dead_code, unused_variables))]

use std::process;
use std::sync::mpsc::{self, TryRecvError};
use std::collections::VecDeque;

use piston_window::{
    PistonWindow,
    WindowSettings,
    G2d,
    context,
    clear,
    line,
    polygon,
};

use app::ReadOnly;
use extensions::ConvertScreenCoordinates;
use state::{Path, Polygon, Pen, TurtleState, DrawingState};
use query::{Query, DrawingCommand};
use event::from_piston_event;
use types::Point;
use color::{self, Color};

#[derive(Debug)]
pub enum Drawing {
    Path(Path),
    Polygon(Polygon),
}

pub struct Renderer {
    drawings: Vec<Drawing>,
    /// Polygon that is currently in the process of being filled
    /// Removed when EndFill is sent
    fill_polygon: Option<(Vec<Path>, Polygon)>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Self {
            drawings: Vec::new(),
            fill_polygon: None,
        }
    }

    pub fn run(&mut self, query_rx: mpsc::Receiver<Query>, state: ReadOnly) {
        let mut window: PistonWindow = WindowSettings::new(
            "Turtle", [800, 600]
        ).exit_on_esc(true).build().unwrap();

        let mut center = [0.0, 0.0];
        let mut events = VecDeque::new();

        'renderloop:
        while let Some(e) = window.next() {
            if let Some(event) = from_piston_event(&e, |pt| pt.to_local_coords(center)) {
                events.push_back(event);
            }

            // Need to handle all of the queries we receive at once so that any lag caused by
            // how long rendering takes doesn't cause any problems
            loop {
                match query_rx.try_recv() {
                    Ok(Query::Request(req)) => unimplemented!(),
                    Ok(Query::Drawing(cmd)) => self.handle_drawing_command(cmd),
                    Err(TryRecvError::Empty) => break, // Do nothing
                    Err(TryRecvError::Disconnected) => break 'renderloop, // Quit
                }
            }

            window.draw_2d(&e, |c, g| {
                let view = c.get_view_size();
                let width = view[0] as f64;
                let height = view[1] as f64;
                center = [width * 0.5, height * 0.5];

                // We clone the relevant state before rendering so that the rendering thread
                // doesn't need to keep locking, waiting or making the main thread wait
                let drawing = state.drawing().clone();
                let temporary_path = state.temporary_path().clone();
                let turtle = state.turtle().clone();

                self.render(c, g, center, &drawing, &temporary_path, &turtle);
            });
        }

        // Quit immediately when the window is closed
        process::exit(0);
    }

    /// Handles a drawing command sent from the main thread
    fn handle_drawing_command(&mut self, command: DrawingCommand) {
        //NOTE: Do not pass the ReadOnly state to this function. By the time a DrawingCommand is
        // handled, that state may be completely out of date
        use self::DrawingCommand::*;
        match command {
            StorePath(path) => {
                if self.fill_polygon.is_some() {
                    let &mut (ref mut border, ref mut poly) = self.fill_polygon.as_mut().unwrap();
                    border.push(path.clone());

                    let Path {start, end, ..} = path;
                    if poly.vertices.last().map_or(true, |&v| v != start) {
                        poly.vertices.push(start);
                    }
                    poly.vertices.push(end);
                }
                else if path.pen.enabled {
                    self.drawings.push(Drawing::Path(path));
                }
            },
            BeginFill(fill_color) => {
                // Calling begin_fill multiple times is okay, it just won't do anything until
                // end_fill is called
                self.fill_polygon = self.fill_polygon.take().or_else(|| Some((Vec::new(), Polygon {
                    vertices: Vec::new(),
                    fill_color: fill_color,
                })));
            },
            // Calling end_fill multiple times is not a problem
            EndFill => if let Some((border, poly)) = self.fill_polygon.take() {
                // Always add the border over the filled polygon so the border is drawn on top
                self.drawings.push(Drawing::Polygon(poly));
                self.drawings.extend(border.into_iter().filter_map(|p| if p.pen.enabled {
                    Some(Drawing::Path(p))
                } else { None }));
            },
            Clear => {
                self.drawings.clear();
                // Because clear doesn't actually shorten the capacity of the vector
                // We don't want to leak memory by leaving this Vec at whatever its size was before
                self.drawings.shrink_to_fit();
                self.fill_polygon.take();
            }
        }
    }

    /// The main rendering route. Dispatches to other functions as needed.
    fn render(&self, c: context::Context, g: &mut G2d, center: Point,
        drawing: &DrawingState, temporary_path: &Option<Path>, turtle: &TurtleState) {
        let background = drawing.background;
        clear(background.into(), g);

        for drawing in &self.drawings {
            match *drawing {
                Drawing::Path(ref path) => self.render_path(c, g, center, path),
                Drawing::Polygon(ref poly) => self.render_polygon(c, g, center,
                    poly.fill_color, poly.vertices.iter()),
            }
        }

        if let Some(&(ref border, ref poly)) = self.fill_polygon.as_ref() {
            // If the temporary_path is not None, we need to add it to the polygon being
            // filled or else the polygon will fall one edge behind in the animation
            let extra = temporary_path.as_ref().map_or(Vec::new(), |&Path {start, end, ..}| {
                if poly.vertices.last().map_or(true, |&v| v != start) {
                    vec![start, end]
                }
                else {
                    vec![end]
                }
            });
            self.render_polygon(c, g, center, poly.fill_color,
                poly.vertices.iter().chain(extra.iter()));

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
    fn render_path(&self, c: context::Context, g: &mut G2d, center: Point, path: &Path) {
        let &Path {start, end, ref pen} = path;
        let &Pen {thickness, color, enabled} = pen;
        debug_assert!(enabled, "bug: attempt to render path when pen was not enabled");

        let start = start.to_screen_coords(center);
        let end = end.to_screen_coords(center);

        line(color.into(), thickness,
            [start[0], start[1], end[0], end[1]],
            c.transform, g);
    }

    /// Render a polygon given its vertices
    fn render_polygon<'a, T: Iterator<Item=&'a Point>>(
        &self,
        c: context::Context,
        g: &mut G2d,
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
        let verts = verts.map(|p| p.to_screen_coords(center)).collect::<Vec<_>>();
        polygon(fill_color.into(), &verts, c.transform, g);
    }

    /// Draw the turtle's shell
    fn render_shell(&self, c: context::Context, g: &mut G2d, center: Point,
        &TurtleState {position, heading, visible, ..}: &TurtleState) {
        // Calculate all the points on the shell by rotating the shell shape by the turtle's
        // heading and moving it to the turtle's position
        if !visible {
            return;
        }

        let cos = heading.cos();
        let sin = heading.sin();
        let turtle_x = position[0];
        let turtle_y = position[1];
        let shell: Vec<_> = [
            [0., 15.],
            [10., 0.],
            [0., -15.],
        ].into_iter().map(|pt| {
            // Rotate each point by the heading and add the current turtle position
            let x = cos * pt[0] - sin * pt[1] + turtle_x;
            let y = sin * pt[0] + cos * pt[1] + turtle_y;
            [x, y].to_screen_coords(center)
        }).collect();

        // Draw the turtle shell with its background first, then its border
        polygon(color::WHITE.into(), &shell, c.transform, g);
        for i in 0..shell.len() {
            let start = shell[i];
            let end = shell[(i + 1) % shell.len()];

            line(color::BLACK.into(), 1.,
            [start[0], start[1], end[0], end[1]],
            c.transform, g);
        }
    }
}
