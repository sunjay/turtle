// During tests, we disable the renderer and that causes a bunch of warnings that we just want
// to get rid of.
// See Cargo.toml for an explanation of this attribute
#![cfg_attr(any(feature = "test", test), allow(dead_code, unused_imports))]

use std::io::{Read, Write};
use std::sync::mpsc;

use query::{Query, Response};
use messenger;

/// Read responses from the renderer process and send them back to the TurtleWindow
pub fn run<R: Read>(renderer_stdout: R, response_tx: mpsc::Sender<Response>) {
    messenger::read_forever(
        renderer_stdout,
        "bug: unable to read data from renderer process",
        "bug: failed to read response from renderer process",
        |resp| response_tx.send(resp).map_err(|_| ()),
    );
}

/// Sends a query to the renderer process
pub fn send_query<W: Write>(renderer_stdin: W, query: &Query) -> Result<(), ()> {
    messenger::send(renderer_stdin, query, "bug: unable to write final newline when sending query")
}
