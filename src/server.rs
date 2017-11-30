use std::io::{self, Write, BufReader, BufRead};
use std::sync::mpsc;

use serde_json;

use app::TurtleApp;
use event::Event;
use query::{Query, DrawingCommand, Request, StateUpdate, Response};

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
    let mut reader = BufReader::new(stdin);
    loop {
        let mut buffer = String::new();
        let read_bytes = reader.read_line(&mut buffer)
            .expect("bug: unable to read data from stdin");
        if read_bytes == 0 {
            // Reached EOF, turtle process must have quit
            // We stop this loop since there is no point in continuing to read from something that
            // will never produce anything again
            break;
        }

        let query: Result<Query, _> = serde_json::from_str(&buffer);
        match query {
            Ok(query) => match query {
                Query::Request(req) => match handle_request(req, &app, &events_rx) {
                    Ok(_) => {},
                    Err(_) => break,
                },
                Query::Update(update) => handle_update(update, &mut app),
                Query::Drawing(cmd) => match drawing_tx.send(cmd) {
                    Ok(_) => {},
                    // The renderer thread is no longer around, so quit
                    Err(_) => break,
                },
            },
            Err(err) => {
                if err.is_io() || err.is_syntax() || err.is_data() {
                    panic!("bug: failed to read command from turtle process");
                }
                else if err.is_eof() {
                    // Could not read anymore bytes from stdin, the turtle process must have ended
                    break;
                }
            },
        }
    }
}

fn handle_request(
    request: Request,
    app: &TurtleApp,
    events_rx: &mpsc::Receiver<Event>,
) -> Result<(), ()> {
    use self::Request::*;
    send_response(&match request {
        TurtleState => Response::TurtleState((*app.turtle()).clone()),
        DrawingState => Response::DrawingState((*app.drawing()).clone()),
        Event => Response::Event(events_rx.recv().map_err(|_| ())?),
    })
}

fn handle_update(
    update: StateUpdate,
    app: &mut TurtleApp,
) {
    use self::StateUpdate::*;
    match update {
        TurtleState(turtle) => *app.turtle_mut() = turtle,
        DrawingState(drawing) => *app.drawing_mut() = drawing,
        TemporaryPath(path) => app.set_temporary_path(path),
    }
}

/// Sends a response to stdout
fn send_response(response: &Response) -> Result<(), ()> {
    let mut stdout = io::stdout();
    match serde_json::to_writer(&mut stdout, response) {
        Ok(_) => {
            writeln!(&mut stdout)
                .expect("bug: unable to write final newline when sending response");
            Ok(())
        },
        Err(err) => {
            if err.is_io() || err.is_eof() {
                Err(())
            }
            else {
                // The other cases for err all have to do with input, so those should never occur
                unreachable!("bug: got an input error when writing output");
            }
        },
    }
}
