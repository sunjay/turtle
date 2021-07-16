//! This is NOT a real example. This is a test designed to see if we can actually run the turtle
//! process
// To run, use the command: cargo run --features unstable --example runtest
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

use turtle::Drawing;

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    turtle.set_speed(2);
    turtle.right(90.0);
    turtle.forward(50.0);

    drawing.destroy();
}
