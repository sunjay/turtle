//! This is NOT a real example. This is a test designed to see if we can actually run the turtle
//! process

extern crate turtle;

use std::panic;
use std::process;

use turtle::Turtle;

fn main() {
    panic::catch_unwind(|| {
        let mut turtle = Turtle::new();

        turtle.set_speed(10);
        turtle.right(90.0);
        turtle.forward(10.0);
        process::exit(0);
    }).unwrap_or_else(|_| {
        // Need to force the thread to exit with exit code 1 because the renderer has some
        // code that will call process::exit(0)
        eprintln!("panic!");
        process::exit(1);
    });
}
