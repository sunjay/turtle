//! This example uses multiple turtles to demonstrate what happens when you clear the drawing while
//! other turtles are in the middle of drawing.
//!
//! Thread 1: Draws a single long line across the top of the window
//!
//! Thread 2: Draws many short lines across the top of the window
//!
//! Thread 3 (main thread): Draws a short line, then clears the entire drawing, then continues
//! drawing
//!
//! When Thread 3 clears the entire drawing, the line from Thread 1 is deleted and the turtle is
//! stopped immediately at its current position. Thread 2 is also stopped, but continues drawing its
//! remaining lines from wherever it was stopped. Note that the lines in Thread 2 will not finish at
//! their intended position. This is because the code is not able to account for the fact that
//! Thread 2 may be interrupted while it is drawing a line.

// To run this example, use the command: cargo run --features unstable --example concurrencytest
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

use std::thread;

use turtle::Drawing;

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle1 = drawing.add_turtle();
    let mut turtle2 = drawing.add_turtle();
    let mut turtle3 = drawing.add_turtle();

    thread::spawn(move || {
        turtle1.pen_up();
        turtle1.set_speed("instant");
        turtle1.go_to((-350.0, 250.0));
        turtle1.set_heading(0.0);
        turtle1.set_speed("normal");
        turtle1.pen_down();

        turtle1.set_pen_color("green");
        turtle1.set_pen_size(5.0);
        turtle1.forward(700.0);
    });

    thread::spawn(move || {
        turtle2.pen_up();
        turtle2.set_speed("instant");
        turtle2.go_to((-350.0, 200.0));
        turtle2.set_heading(0.0);
        turtle2.set_speed("faster");
        turtle2.pen_down();

        turtle2.set_pen_size(5.0);
        for i in 0..20 {
            let color = if i % 2 == 0 {
                "purple"
            } else {
                "pink"
            };
            turtle2.set_pen_color(color);
            turtle2.forward(35.0);
        }
    });

    turtle3.pen_up();
    turtle3.set_speed("instant");
    turtle3.backward(250.0);
    turtle3.set_speed("normal");
    turtle3.pen_down();

    turtle3.set_pen_color("blue");
    turtle3.set_pen_size(5.0);

    turtle3.forward(200.0);
    drawing.clear();
    turtle3.set_pen_color("red");
    turtle3.forward(200.0);

    //TODO: Currently, if the main thread ends before the other threads, the window just closes
    thread::park();
}
