use std::process;
use std::sync::mpsc::{self, TryRecvError};

use piston_window::{
    PistonWindow,
    G2d,
    context,
    clear,
    line,
    polygon,
};

use turtle_window::ReadOnly;
use extensions::ConvertScreenCoordinates;
use state::{Path, Polygon, Pen, TurtleState};
use event::from_piston_event;
use {Point, Event, color};

pub enum DrawingCommand {
    /// When a path is finished being animated, it needs to be persisted in the renderer
    /// so it can be redrawn every frame
    StorePath(Path),
    /// Begins filling with the current fill color from the next path onwards. If temporary_path is
    /// set, it is included in the fill shape. Any paths sent via StorePath will be added to the
    /// filled shape.
    BeginFill,
    /// Send EndFill to finish the filled shape.
    EndFill,
    /// Clears the image completely
    ///
    /// Panics if temporary_path is not None
    Clear,
}

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

    pub fn run(
        &mut self,
        window: &mut PistonWindow,
        drawing_rx: mpsc::Receiver<DrawingCommand>,
        events_tx: mpsc::Sender<Event>,
        state: ReadOnly,
    ) {
        let mut center = [0.0, 0.0];

        'renderloop:
        while let Some(e) = window.next() {
            if let Some(event) = from_piston_event(&e, |pt| pt.to_local_coords(center)) {
                match events_tx.send(event) {
                    Ok(_) => {},
                    // Quit
                    Err(_) => break,
                }
            }

            loop {
                match drawing_rx.try_recv() {
                    Ok(cmd) => self.handle_drawing_command(cmd, &state),
                    Err(TryRecvError::Empty) => break, // Do nothing
                    Err(TryRecvError::Disconnected) => break 'renderloop, // Quit
                }
            }

            window.draw_2d(&e, |c, g| {
                let background = state.drawing().background;
                clear(background.into(), g);

                let view = c.get_view_size();
                let width = view[0] as f64;
                let height = view[1] as f64;
                center = [width * 0.5, height * 0.5];

                for drawing in &self.drawings {
                    match *drawing {
                        Drawing::Path(ref path) => self.render_path(c, g, center, path),
                        Drawing::Polygon(ref poly) => self.render_polygon(c, g, center, poly),
                    }
                }

                if let Some(&(ref border, ref poly)) = self.fill_polygon.as_ref() {
                    self.render_polygon(c, g, center, poly);
                    for path in border {
                        self.render_path(c, g, center, path);
                    }
                }
                //TODO: Render the temporary_path as part of the polygon when fill_polygon.is_some()

                if let Some(ref path) = *state.temporary_path() {
                    if path.pen.enabled {
                        self.render_path(c, g, center, path);
                    }
                }

                self.render_shell(c, g, center, &state);
            });
        }

        // Quit immediately when the window is closed
        process::exit(0);
    }

    /// Handles a drawing command sent from the main thread
    fn handle_drawing_command(&mut self, command: DrawingCommand, state: &ReadOnly) {
        use self::DrawingCommand::*;
        match command {
            StorePath(path) => {
                if self.fill_polygon.is_some() {
                    let &mut (ref mut border, ref mut poly) = self.fill_polygon.as_mut().unwrap();
                    border.push(path.clone());

                    let Path {start, end, ..} = path;
                    if poly.vertices.last().map_or(true, |&v| v != path.start) {
                        poly.vertices.push(start);
                    }
                    poly.vertices.push(end);
                }
                else if path.pen.enabled {
                    self.drawings.push(Drawing::Path(path));
                }
            },
            BeginFill => {
                // Calling begin_fill multiple times is okay, it just won't do anything until
                // end_fill is called
                self.fill_polygon = self.fill_polygon.take().or_else(|| Some((Vec::new(), Polygon {
                    vertices: Vec::new(),
                    fill_color: state.drawing().fill_color,
                })));
            },
            // Calling end_fill multiple times is not a problem
            EndFill => if let Some((border, poly)) = self.fill_polygon.take() {
                // Always add the border over the filled polygon so the border is drawn on top
                self.drawings.push(Drawing::Polygon(poly));
                self.drawings.extend(border.into_iter().map(Drawing::Path));
            },
            Clear => {
                assert!(state.temporary_path().is_none());
                unimplemented!(); //TODO
            }
        }
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

    /// Render a polygon
    fn render_polygon(&self, c: context::Context, g: &mut G2d, center: Point, poly: &Polygon) {
        let verts = poly.vertices.iter().map(|p| p.to_screen_coords(center)).collect::<Vec<_>>();
        polygon(poly.fill_color.into(), &verts, c.transform, g);
    }

    /// Draw the turtle's shell
    fn render_shell(&self, c: context::Context, g: &mut G2d, center: Point, state: &ReadOnly) {
        // Calculate all the points on the shell by rotating the shell shape by the turtle's
        // heading and moving it to the turtle's position
        let TurtleState {position, heading, visible, ..} = *state.turtle();
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
