// Draw a dragon curve, more specifically a highway dragon.
// (https://en.wikipedia.org/wiki/Dragon_curve)

extern crate turtle;

use turtle::Turtle;
use turtle::Color;

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_background_color("#112244");

    turtle.pen_up();
    turtle.backward(160.0);
    turtle.left(90.);
    turtle.forward(110.);
    turtle.pen_down();
    turtle.set_speed(10);

    dragon(&mut turtle, 90., 11, 0., 255.);

    turtle.hide();
}

fn dragon(turtle: &mut Turtle, turn: f64, depth: usize, c0: f64, c1: f64) {
    let cmid = (c0 + c1) * 0.5;
    if depth == 0 {
        turtle.set_pen_color(Color {
            red: (cmid - 128.).abs() * 2.,
            green: cmid,
            blue: 160.,
            alpha: 1.,
        });
        return turtle.forward(10.);
    }

    dragon(turtle, 90., depth - 1, c0, cmid);
    turtle.right(turn);
    dragon(turtle, -90., depth - 1, cmid, c1);
}
