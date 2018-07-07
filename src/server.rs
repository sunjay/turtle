#[cfg(target_arch = "wasm32")]
compile_error!("This module should not be included when compiling to wasm");

use std::env;
use std::thread;
use std::process;
use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};

use messenger::{self, Disconnected};
use app::TurtleApp;
use renderer::Renderer;
use query::{Query, DrawingCommand, Request, StateUpdate, Response};
use {Event};

/// Start the turtle window in advance
///
/// If you do not create a turtle immediately at the beginning of `main()` with [`Turtle::new()`],
/// you must **call this function at the start of `main()` to avoid any problems**.
///
/// Since the majority of code created using this crate does little or no work before calling
/// `Turtle::new()`, this usually isn't a problem. Programs that parse command line arguments, read
/// input, or check environment variables may **fail** to start if this function is not called
/// right at the beginning of the program. Programs that perform any expensive computations may
/// experience delayed start up problems unless they call this function first.
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
///     turtle::start();
///
///     // Do all kinds of expensive work here...
///     // Feel free to check environment variables, command line arguments, etc.
///
///     // Create the turtle when you are ready
///     // Turtle::new() will also call start(), but calling it twice doesn't matter
///     let mut turtle = Turtle::new();
///     // Do things with the turtle...
/// }
/// ```
///
/// [`Turtle::new()`]: struct.Turtle.html#method.new
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
    Renderer::new(app).run(drawing_rx, events_tx);

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
) -> Result<Option<Response>, Disconnected> {
    handle_query(query, app, events_rx, drawing_tx)
}

/// Returns the appropriate Response (if any) to the given Query
fn handle_query(
    query: Query,
    app: &mut TurtleApp,
    events_rx: &mpsc::Receiver<Event>,
    drawing_tx: &mpsc::Sender<DrawingCommand>,
) -> Result<Option<Response>, Disconnected> {
    match query {
        Query::Request(req) => handle_request(req, &app, &events_rx),
        Query::Update(update) => handle_update(update, app),
        Query::Drawing(cmd) => drawing_tx.send(cmd).map(|_| None).map_err(|_| Disconnected),
    }
}

fn handle_request(
    request: Request,
    app: &TurtleApp,
    events_rx: &mpsc::Receiver<Event>,
) -> Result<Option<Response>, Disconnected> {
    use self::Request::*;
    Ok(Some(match request {
        TurtleState => Response::TurtleState((*app.turtle()).clone()),
        DrawingState => Response::DrawingState((*app.drawing()).clone()),
        Event => Response::Event(match events_rx.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => return Err(Disconnected),
        }),
    }))
}

fn handle_update(
    update: StateUpdate,
    app: &mut TurtleApp,
) -> Result<Option<Response>, Disconnected> {
    use self::StateUpdate::*;
    match update {
        TurtleState(turtle) => *app.turtle_mut() = turtle,
        DrawingState(drawing) => *app.drawing_mut() = drawing,
        TemporaryPath(path) => app.set_temporary_path(path),
    }

    Ok(None)
}

/// Sends a response to stdout
fn send_response<W: Write>(writer: W, response: &Response) -> Result<(), Disconnected> {
    messenger::send(writer, response, "bug: unable to write final newline when sending response")
}
