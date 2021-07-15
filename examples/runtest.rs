//! This is NOT a real example. This is a test designed to see if we can actually run the turtle
//! process

use turtle::Drawing;

fn main() {
    let mut drawing = Drawing::new();

    let mut turtle = drawing.add_turtle();

    turtle.set_speed(2);
    turtle.right(90.0);
    turtle.forward(50.0);

    drawing.destroy();
}
