use std::io::{Read, Write};
use std::sync::mpsc;

use serde_json;

use query::{Query, Response};

/// Read responses from the renderer process and send them back to the TurtleWindow
pub fn run<R: Read>(mut renderer_stdout: R, response_tx: mpsc::Sender<Response>) {
    loop {
        match serde_json::from_reader(&mut renderer_stdout) {
            Ok(response) => match response_tx.send(response) {
                Ok(_) => {},
                // The main thread is no longer around, so quit
                Err(_) => break,
            },
            Err(err) => {
                if err.is_io() || err.is_syntax() || err.is_data() {
                    panic!("bug: failed to read response from renderer process");
                }
                else if err.is_eof() {
                    // Could not read anymore bytes from stdin, the renderer process must have ended
                    break;
                }
            },
        }
    }
}

/// Sends a query to the renderer process
pub fn send_query<W: Write>(renderer_stdin: W, query: &Query) -> Result<(), ()> {
    match serde_json::to_writer(renderer_stdin, query) {
        Ok(_) => Ok(()),
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
