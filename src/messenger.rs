//! Utilities for I/O shared by the client and server

use std::io::{Read, Write, BufReader, BufRead};

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::error::Category;
use serde_json;

/// Continuously read from something that implements Read
/// For each line of input, try to read that line and serialize
/// it from JSON. Pass the result into a function.
///
/// If that function returns an Err(()), break the loop. Otherwise continue to read until EOF.
pub fn read_forever<R: Read, T: DeserializeOwned, F: FnMut(T) -> Result<(), ()>>(
    reader: R,
    unable_to_read_bytes: &'static str,
    failed_to_read_result: &'static str,
    mut handler: F,
) {
    let mut reader = BufReader::new(reader);
    loop {
        let mut buffer = String::new();
        let read_bytes = reader.read_line(&mut buffer)
            .expect(unable_to_read_bytes);
        if read_bytes == 0 {
            // Reached EOF, renderer process must have quit
            break;
        }

        let result = serde_json::from_str(&buffer).map_err(|err| match err.classify() {
            Category::Io | Category::Syntax | Category::Data => panic!(failed_to_read_result),
            Category::Eof => (),
        }).and_then(|result| handler(result));
        if result.is_err() {
            break;
        }
    }
}

/// Writes a message to given Write stream.
pub fn send<W: Write, T: Serialize>(
    mut writer: W,
    message: &T,
    unable_to_write_newline: &str,
) -> Result<(), ()> {
    serde_json::to_writer(&mut writer, message).map_err(|err| match err.classify() {
        Category::Io | Category::Eof => (),
        // The other cases for err all have to do with input, so those should never occur
        Category::Syntax | Category::Data => {
            unreachable!("bug: got an input error when writing output")
        },
    }).map(|_| writeln!(writer).expect(unable_to_write_newline))
}
