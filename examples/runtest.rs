//! This is NOT a real example. This is a test designed to see if we can actually run the turtle
//! process

#[macro_use]
extern crate turtle;

use std::process;

run_turtle!(|mut turtle| {

    turtle.set_speed(2);
    turtle.right(90.0);
    turtle.forward(50.0);
    process::exit(0);
});
