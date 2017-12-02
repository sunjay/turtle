use std::env;
use std::thread;
use std::process;
use std::sync::mpsc::{self, TryRecvError};

use piston_window::{
    PistonWindow,
    WindowSettings,
    G2d,
    context,
    clear,
    line,
    polygon,
};

use server;
use app::{TurtleApp, ReadOnly};
use event::from_piston_event;
use extensions::ConvertScreenCoordinates;
use query::DrawingCommand;
use state::{Path, Polygon, Pen, TurtleState, DrawingState};
use {Point, Event, Color, color};

/// Setup the turtle window in advance
///
/// You must call this function at the beginning of `main()` if you do not create the
/// turtle immediately using [`Turtle::new()`].
///
/// It's a good idea to call this function before any other code runs in `main()`. Programs that
/// parse command line arguments or look at environment variables may **fail** to start if this
/// function is not called right at the beginning of the program. Programs that perform any
/// expensive computations may experience delayed start up problems unless they call this
/// function first.
///
/// The [`Turtle::new()`] method will call this function for you so that you don't need to worry
/// about this unless you are doing something before that.
///
/// # Example
/// ```rust,no_run
/// # #![allow(unused_variables, unused_mut)]
/// extern crate turtle;
/// use turtle::Turtle;
///
/// fn main() {
///     // Initializes the turtle renderer first so that there is less delay when a Turtle
///     // is created and so that there are no conflicts with command line arguments or
///     // environment variables.
///     // Not required if Turtle::new() is already at the top of main.
///     turtle::setup();
///
///     // Do all kinds of expensive work here...
///     // Feel free to check environment variables, command line arguments, etc.
///
///     // Create the turtle when you are ready
///     // Turtle::new() will also call setup(), but calling it twice doesn't matter
///     let mut turtle = Turtle::new();
///     // Do things with the turtle...
/// }
/// ```
///
/// [`Turtle::new()`]: struct.Turtle.html#method.new
pub fn setup() {
    // If this environment variable is present, this process is taken over so that no other
    // code runs after main(). This allows us to ship one executable that appears to
    // have two separate processes.
    // We run the renderer loop and then immediately exit.
    if env::var("RUN_TURTLE_CANVAS").unwrap_or_else(|_| "".to_owned()) == "true" {
        // This code MUST be run on the main thread.

        // Run the renderer process
        main();
        unreachable!("bug: renderer loop did not exit after finishing");
    }
}

/// Run the renderer process in the current thread
///
/// This function must run in the main thread ONLY
pub fn main() {
    let app = TurtleApp::new();
    let read_only = app.read_only();
    let (drawing_tx, drawing_rx) = mpsc::channel();
    let (events_tx, events_rx) = mpsc::channel();

    // The running channel is entirely for checking if the other thread is still active.
    // We need to check because while we can exit while that thread is still running, we want
    // any errors caused in that thread to be reported.
    let (running_tx, running_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        server::run(app, drawing_tx, events_rx, running_tx);
    });

    // Renderer MUST run on the main thread or else it will panic on MacOS
    Renderer::new().run(drawing_rx, events_tx, read_only);

    // Quit immediately when the window is closed

    // Check if an error has occurred on the thread
    match running_rx.try_recv() {
        Ok(_) => unreachable!("bug: running channel should always be empty"),
        // The thread was still running, exit normally
        Err(mpsc::TryRecvError::Empty) => process::exit(0),
        Err(mpsc::TryRecvError::Disconnected) => match handle.join() {
            Ok(_) => process::exit(0),
            // The other thread must have panicked
            Err(_) => process::exit(1),
        },
    }
}

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

    pub fn run(
        &mut self,
        drawing_rx: mpsc::Receiver<DrawingCommand>,
        events_tx: mpsc::Sender<Event>,
        state: ReadOnly,
    ) {
        // This check isn't foolproof. Someone can always create a thread named "main".
        if thread::current().name().unwrap_or("") != "main" {
            // In order to maintain compatibility with MacOS, we need to make sure that windows are
            // only created on the main thread. We do this check on all platforms so that no one
            // can accidentally make a change that creates the window off of the main thread.
            unreachable!("bug: windows can only be created on the main thread");
        }
        let mut window: PistonWindow = WindowSettings::new(
            "Turtle", [800, 600]
        ).exit_on_esc(true).build().unwrap();

        let mut center = [0.0, 0.0];

        'renderloop:
        while let Some(e) = window.next() {
            if let Some(event) = from_piston_event(&e, |pt| pt.to_local_coords(center)) {
                match events_tx.send(event) {
                    Ok(_) => {},
                    // Quit - the server thread must have quit
                    Err(_) => break,
                }
            }

            // Need to handle all of the queries we receive at once so that any lag caused by
            // how long rendering takes doesn't cause any problems
            loop {
                match drawing_rx.try_recv() {
                    Ok(cmd) => self.handle_drawing_command(cmd),
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
