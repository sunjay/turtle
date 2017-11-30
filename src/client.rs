use std::io::{Read, Write, BufReader, BufRead};
use std::sync::mpsc;

use serde_json;

use query::{Query, Response};

/// Read responses from the renderer process and send them back to the TurtleWindow
pub fn run<R: Read>(renderer_stdout: R, response_tx: mpsc::Sender<Response>) {
    let mut reader = BufReader::new(renderer_stdout);
    loop {
        let mut buffer = String::new();
        let read_bytes = reader.read_line(&mut buffer)
            .expect("bug: unable to read data from renderer process");
        if read_bytes == 0 {
            // Reached EOF, renderer process must have quit
            break;
        }

        let response: Result<Response, _> = serde_json::from_str(&buffer);
        match response {
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
pub fn send_query<W: Write>(mut renderer_stdin: W, query: &Query) -> Result<(), ()> {
    match serde_json::to_writer(&mut renderer_stdin, query) {
        Ok(_) => {
            writeln!(renderer_stdin)
                .expect("bug: unable to write final newline when sending query");
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
