//! This is NOT a real example. This is a test designed to see if we can actually run the turtle
//! process

use std::process;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_speed(2);
    turtle.right(90.0);
    turtle.forward(50.0);

    //TODO: Exiting the process currently doesn't cause the window to get closed. We should add a
    // `close(self)` or `quit(self)` method to `Drawing` that closes the window explicitly.
    process::exit(0);
}
