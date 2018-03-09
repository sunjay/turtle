extern crate turtle;

use turtle::Turtle;

/// This program draws several parallel lines to demonstrate each of the
/// different possible movement speeds
fn main() {
    let mut turtle = Turtle::new();

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.left(90.0);
    turtle.forward(350.0);
    turtle.right(90.0);
    turtle.pen_down();

    let length = 200.0;

    for i in 1..25 {
        turtle.set_speed(i);
        turtle.forward(length);

        turtle.pen_up();
        turtle.set_speed("instant");
        turtle.backward(length);
        turtle.right(90.0);
        turtle.forward(30.0);
        turtle.left(90.0);
        turtle.pen_down();
    }

    turtle.hide();
}
