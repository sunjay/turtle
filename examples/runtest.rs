//! This is NOT a real example. This is a test designed to see if we can actually run the turtle
//! process

extern crate turtle;

use std::process;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_speed(2);
    turtle.right(90.0);
    turtle.forward(50.0);
    process::exit(0);
}
