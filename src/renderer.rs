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
use extensions::ToCanvasCoordinates;
use {Point, Color};
use state::{Path, Pen,TurtleState};

pub struct Renderer {
    paths: Vec<Path>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Self {
            paths: Vec::new(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, paths_rx: mpsc::Receiver<Path>, state: ReadOnly) {
        'renderloop:
        while let Some(e) = window.next() {
            loop {
                match paths_rx.try_recv() {
                    Ok(path) => if path.pen.enabled {
                        self.paths.push(path);
                    },
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
                let center = [width * 0.5, height * 0.5];

                for path in &self.paths {
                    self.render_path(c, g, center, path);
                }

                if let Some(ref path) = *state.temporary_path() {
                    self.render_path(c, g, center, path);
                }

                self.render_shell(c, g, center, &state);
            });
        }

        // Quit immediately when the window is closed
        process::exit(0);
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
    fn render_shell(&self, c: context::Context, g: &mut G2d, center: Point, state: &ReadOnly) {
        // Calculate all the points on the shell by rotating the shell shape by the turtle's
        // heading and moving it to the turtle's position
        let TurtleState {position, heading, ..} = *state.turtle();
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
