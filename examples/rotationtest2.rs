extern crate turtle;

use turtle::Turtle;

/// This program demonstrates rotations at multiple speeds
fn main() {
    let mut turtle = Turtle::new();

    turtle.pen_up();
    turtle.set_speed("fastest");
    turtle.left(90.0);
    turtle.forward(300.0);
    turtle.right(180.0);
    turtle.pen_down();

    turtle.hide();
    for i in 1..12 {
        turtle.set_speed(i);
        circle(&mut turtle);

        turtle.pen_up();
        turtle.set_speed("fastest");
        turtle.forward(60.0);
        turtle.pen_down();
    }
}

fn circle(turtle: &mut Turtle) {
    for _ in 0..180 {
        turtle.forward(1.0);
        turtle.left(2.0);
    }
}
