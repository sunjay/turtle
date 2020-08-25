//! This example uses multiple turtles to demonstrate the sequential consistency guarantees of this
//! crate.
//!
//! Turtle 1: Draws a single long line across the top of the window
//!
//! Turtle 2: Draws many short lines across the top of the window
//!
//! Turtle 3: Draws a short line, then clears the image (including the lines from Turtle 1 and 2),
//!   then continues drawing
//!
//! If Turtle 1 has already started drawing before Turtle 3 clears the image, Turtle 3 will have to
//! wait for Turtle 1 to finish drawing its line before it can clear the image and keep drawing.
//! This is because Turtle 1 has exclusive access to its turtle while it is drawing. The clear
//! command cannot execute until that line has finished drawing and it is given access.
//!
//! Turtle 2 also stops drawing when the drawing is cleared, but for different reasons than
//! Turtle 3. Turtle 2 can technically still continue because it isn't blocked on a clear command.
//! The issue is that when clear is run, it reserves all turtles, including Turtle 2. Turtle 2 has
//! to wait for the clear command to run before it can draw its next line. This is how we ensure
//! that commands run in the order they are executed (sequential consistency). No command gets
//! precedence just because the resources it needs are available.

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

    turtle3.forward(100.0);
    drawing.clear();
    turtle3.set_pen_color("red");
    turtle3.forward(100.0);

    //TODO: Currently, if the main thread ends before the other threads, the window just closes
    thread::park();
}
