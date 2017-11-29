use std::io::{self, BufReader, BufRead};
use std::sync::mpsc;

use serde_json;

use app::TurtleApp;
use query::{Query, DrawingCommand};

/// Continuously read queries from stdin and send them to the renderer
pub fn run(app: TurtleApp, drawing_tx: mpsc::Sender<DrawingCommand>) {
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
