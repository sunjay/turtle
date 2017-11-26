use std::io;
use std::process;
use std::sync::mpsc;

use serde_json;

use app::TurtleApp;
use query::{self, Query, Response, DrawingCommand};

/// Continuously read queries from stdin and send them to the renderer
pub fn run(_app: TurtleApp, drawing_tx: mpsc::Sender<DrawingCommand>) {
    loop {
        // Read queries from the turtle process
        let stdin = io::stdin();
        match serde_json::from_reader(stdin) {
            Ok(query) => match query {
                Query::Request(req) => unimplemented!(),
                Query::Update(update) => unimplemented!(),
                Query::Drawing(cmd) => match drawing_tx.send(cmd) {
                    Ok(_) => {},
                    // The renderer thread is no longer around, so quit
                    Err(_) => break,
                },
            },
            Err(err) => {
                if err.is_io() || err.is_syntax() || err.is_data() {
                    let response: Response = Err(query::Error::SyntaxError {
                        line: err.line(),
                        column: err.column(),
                    });
                    send_response(&response);
                }
                else if err.is_eof() {
                    // Could not read anymore bytes from stdin, the turtle process must have ended
                    break;
                }
            },
        }
    }
}

/// Sends a response to the turtle process
pub fn send_response(response: &Response) {
    let stdout = io::stdout();
    match serde_json::to_writer(stdout, response) {
        Ok(_) => {},
        // We could not write to the output stream, so we should probably quit because it is likely
        // that the turtle process has ended
        Err(err) => {
            if err.is_io() || err.is_eof() {
                process::exit(0)
            }
            else {
                // The other cases for err all have to do with input, so those should never occur
                unreachable!("bug: got an input error when writing output");
            }
        },
    }
}
