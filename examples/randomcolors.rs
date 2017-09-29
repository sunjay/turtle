extern crate turtle;

use turtle::{Turtle, Color, random};

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_speed(10);
    turtle.set_pen_size(2.0);
    for i in 0..300 {
        turtle.set_background_color(random::<Color>().visible());

        turtle.set_pen_color(random::<Color>().visible());
        turtle.forward(5.0 + i as f64);

        turtle.right(30.0);
    }
}
