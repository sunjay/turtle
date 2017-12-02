use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};

use messenger;
use app::TurtleApp;
use query::{Query, DrawingCommand, Request, StateUpdate, Response};
use {Event};

/// Continuously read queries from stdin and send them to the renderer
pub fn run(
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
