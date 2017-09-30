extern crate turtle;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_speed(7);
    turtle.set_pen_size(3.0);
    turtle.set_pen_color("red");

    turtle.pen_up();
    turtle.forward(-50.0);
    turtle.pen_down();

    turtle.left(50.0);
    turtle.forward(111.65);
    turtle.set_speed(10);
    curve(&mut turtle);
    turtle.left(120.0);
    curve(&mut turtle);
    turtle.set_speed(7);
    turtle.forward(111.65);

    turtle.forward(20.0);
    for _ in 0..10 {
        turtle.right(2.0);
        turtle.forward(2.0);
    }

    turtle.set_background_color("pink");
}

fn curve(turtle: &mut Turtle) {
    for _ in 0..100 {
        turtle.right(2.0);
        turtle.forward(2.0);
    }
}
