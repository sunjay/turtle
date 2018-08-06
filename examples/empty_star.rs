extern crate turtle;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    let points = 5.0;
    let angle = 180.0 / points;

    turtle.drawing_mut().set_background_color("#424242");
    turtle.set_pen_size(2.0);
    turtle.set_pen_color("yellow");

    turtle.pen_up();
    turtle.forward(150.0);
    turtle.right(180.0 - angle / 2.0);
    turtle.pen_down();

    for _ in 0..5 {
        turtle.forward(100.0);
        turtle.left(angle * 2.0);
        turtle.forward(100.0);
        turtle.right(180.0 - angle);
    }
}
