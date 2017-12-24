use std::env;
use std::thread;
use std::process;
use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};

use piston_window::{
    AdvancedWindow,
    PistonWindow,
    WindowSettings,
};

use input::{
    Event as PistonEvent,
    Input, ButtonArgs,
    ButtonState,
    Button,
    Motion,
};

use super::messenger;
use app::TurtleApp;
use renderer::Renderer;
use query::{Query, DrawingCommand, Request, StateUpdate, Response};
use {Event};
use state::DrawingState;
use extensions::ConvertScreenCoordinates;

/// If this process is the child rendering process, enter the rendering loop and do not pass
/// control to user turtle-navigating code.
pub fn start() {
    // If this environment variable is present, this process is taken over so that no other
    // code runs after main(). This allows us to ship one executable that appears to
    // have two separate processes.
    // This implementation detail is why we request that users run start() at the beginning of
    // their programs. When we spawn the same executable, we don't pass along any environment,
    // input or command line arguments. That means that the user *needs* to run start() first or
    // else their program won't be able to run at all. This is a tradeoff of this design decision.
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
fn main() {
    let app = TurtleApp::new();
    let (drawing_tx, drawing_rx) = mpsc::channel();
    let (events_tx, events_rx) = mpsc::channel();

    // The running channel is entirely for checking if the other thread is still active.
    // We need to check because while we can exit while that thread is still running, we want
    // any errors caused in that thread to be reported.
    let (running_tx, running_rx) = mpsc::channel();

    let thread_app = app.clone();
    let handle = thread::spawn(move || {
        read_queries_forever(thread_app, drawing_tx, events_rx, running_tx);
    });

    // Renderer MUST run on the main thread or else it will panic on MacOS
    run_render_loop(drawing_rx, events_tx, app);

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

/// Continuously read queries from stdin and send them to the renderer
fn read_queries_forever(
    mut app: TurtleApp,
    drawing_tx: mpsc::Sender<DrawingCommand>,
    events_rx: mpsc::Receiver<Event>,
    // Intentionally unused. Only used to tell if thread has already quit.
    _running_tx: mpsc::Sender<()>,
) {
    // Read queries from the turtle process
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    messenger::read_forever(
        stdin,
        "bug: unable to read data from stdin",
        "bug: failed to read command from turtle process",
        |query| handle_query(query, &mut app, &events_rx, &drawing_tx).and_then(|resp| match resp {
            Some(ref response) => send_response(&mut stdout, response),
            None => Ok(()),
        }),
    );
}

/// We want to expose this specifically for tests so that tests can mimic the behaviour of the
/// server without actually spawning a new process. See renderer_process.rs for more details.
#[cfg(any(feature = "test", test))]
pub(crate) fn handle_query_for_test_use_only(
    query: Query,
    app: &mut TurtleApp,
    events_rx: &mpsc::Receiver<Event>,
    drawing_tx: &mpsc::Sender<DrawingCommand>,
) -> Result<Option<Response>, ()> {
    handle_query(query, app, events_rx, drawing_tx)
}

/// Returns the appropriate Response (if any) to the given Query
///
/// Returns Err(()) if it is time to quit because we have been disconnected.
fn handle_query(
    query: Query,
    app: &mut TurtleApp,
    events_rx: &mpsc::Receiver<Event>,
    drawing_tx: &mpsc::Sender<DrawingCommand>,
) -> Result<Option<Response>, ()> {
    match query {
        Query::Request(req) => handle_request(req, &app, &events_rx),
        Query::Update(update) => handle_update(update, app),
        Query::Drawing(cmd) => drawing_tx.send(cmd).map(|_| None).map_err(|_| ()),
    }
}

fn handle_request(
    request: Request,
    app: &TurtleApp,
    events_rx: &mpsc::Receiver<Event>,
) -> Result<Option<Response>, ()> {
    use self::Request::*;
    Ok(Some(match request {
        TurtleState => Response::TurtleState((*app.turtle()).clone()),
        DrawingState => Response::DrawingState((*app.drawing()).clone()),
        Event => Response::Event(match events_rx.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => return Err(()),
        }),
    }))
}

fn handle_update(
    update: StateUpdate,
    app: &mut TurtleApp,
) -> Result<Option<Response>, ()> {
    use self::StateUpdate::*;
    match update {
        TurtleState(turtle) => *app.turtle_mut() = turtle,
        DrawingState(drawing) => *app.drawing_mut() = drawing,
        TemporaryPath(path) => app.set_temporary_path(path),
    }

    Ok(None)
}

/// Sends a response to stdout
fn send_response<W: Write>(writer: W, response: &Response) -> Result<(), ()> {
    messenger::send(writer, response, "bug: unable to write final newline when sending response")
}

fn update_window(window: &mut PistonWindow, current: DrawingState, next: DrawingState) -> DrawingState {
    if next.title != current.title {
        window.set_title(next.title.clone());
    }
    if next.width != current.width || next.height != current.height {
        window.window.window.set_inner_size(next.width, next.height);
    }
    if next.maximized != current.maximized {
        window.window.window.set_maximized(next.maximized);
    }
    if next.fullscreen != current.fullscreen {
        if next.fullscreen {
            window.window.window.set_fullscreen(Some(window.window.window.get_current_monitor()));
        } else {
            window.window.window.set_fullscreen(None);
        }
    }
    next
}

fn run_render_loop(drawing_rx: mpsc::Receiver<DrawingCommand>,
                   events_tx: mpsc::Sender<Event>,
                   mut app: TurtleApp) {
    let state = app.read_only();

    let mut renderer = Renderer::new();

    // This check isn't foolproof. Someone can always create a thread named "main".
    if thread::current().name().unwrap_or("") != "main" {
        // In order to maintain compatibility with MacOS, we need to make sure that windows are
        // only created on the main thread. We do this check on all platforms so that no one
        // can accidentally make a change that creates the window off of the main thread.
        unreachable!("bug: windows can only be created on the main thread");
    }
    let mut window: PistonWindow = WindowSettings::new(
        &*state.drawing().title,
        [800, 600]
    ).exit_on_esc(true).build().unwrap();
    // We keep a copy of the DrawingState so that we can tell when it is updated and we need
    // to change something on the window
    let mut current_drawing = DrawingState::default();

    let mut center = state.drawing().center;

    'renderloop:
        while let Some(event) = window.next() {
        match event {
            PistonEvent::Input(Input::Resize(width, height)) => {
                let mut drawing = app.drawing_mut();
                drawing.width = width;
                drawing.height = height;
            },
            _ => {},
        }

        if let Some(event) = from_piston_event(&event, |pt| pt.to_local_coords(center)) {
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
                Ok(cmd) => renderer.handle_drawing_command(cmd),
                Err(TryRecvError::Empty) => break, // Do nothing
                Err(TryRecvError::Disconnected) => break 'renderloop, // Quit
            }
        }

        // Update the window based on any changes in the DrawingState
        current_drawing = update_window(&mut window, current_drawing, state.drawing().clone());

        window.draw_2d(&event, |c, g| {
            let view = c.get_view_size();
            let width = view[0] as f64;
            let height = view[1] as f64;
            center = state.drawing().center.to_screen_coords([width * 0.5, height * 0.5]);

            // We clone the relevant state before rendering so that the rendering thread
            // doesn't need to keep locking, waiting or making the main thread wait
            let drawing = state.drawing().clone();
            let temporary_path = state.temporary_path().clone();
            let turtle = state.turtle().clone();

            renderer.render(c, g, center, &drawing, &temporary_path, &turtle);
        });
    }
}

/// Attempts to convert a piston Event to our event type
fn from_piston_event<F>(event: &PistonEvent, to_local_coords: F) -> Option<Event>
    where F: FnOnce(::Point) -> ::Point {
    use self::Event::*;

    let input_event = match *event {
        PistonEvent::Input(ref input_event) => input_event,
        _ => return None,
    };

    Some(match *input_event {
        Input::Button(ButtonArgs {state, button, scancode: _}) => match state {
            ButtonState::Press => match button {
                Button::Keyboard(key) => KeyPressed(key),
                Button::Mouse(button) => MouseButtonPressed(button),
                Button::Controller(button) => ControllerButtonPressed(button),
            },
            ButtonState::Release => match button {
                Button::Keyboard(key) => KeyReleased(key),
                Button::Mouse(button) => MouseButtonReleased(button),
                Button::Controller(button) => ControllerButtonReleased(button),
            },
        },
        Input::Move(motion) => match motion {
            Motion::MouseCursor(x, y) => {
                let local = to_local_coords([x, y]);
                MouseMove {x: local[0], y: local[1]}
            },
            // Ignored in favor of MouseCursor
            Motion::MouseRelative(..) => return None,
            Motion::MouseScroll(x, y) => MouseScroll {x, y},
            Motion::ControllerAxis(axis) => ControllerAxisChange(axis),
            Motion::Touch(touch) => Touch(touch),
        },
        // Ignored because this value doesn't produce text reliably for all keys
        // (especially when ctrl is pressed)
        Input::Text(_) => return None,
        Input::Resize(width, height) => WindowResized {width, height},
        Input::Focus(focused) => WindowFocused(focused),
        Input::Cursor(cursor) => WindowCursor(cursor),
        Input::Close(_) => WindowClosed,
    })
}
