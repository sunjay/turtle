use std::io;
use std::sync::mpsc;

use serde_json;

use app::TurtleApp;
use query::{Query, DrawingCommand};

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
