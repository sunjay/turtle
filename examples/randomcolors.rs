#[macro_use]
extern crate turtle;

use turtle::rand::Rng;
use turtle::{Color};

run_turtle!(|mut turtle| {

    turtle.set_speed(8);
    turtle.set_pen_size(2.0);
    let mut rng = turtle.rng();
    for i in 0..300 {
        turtle.set_background_color(rng.gen::<Color>().opaque());

        turtle.set_pen_color(rng.gen::<Color>().opaque());
        turtle.forward(i as f64);

        turtle.right(60.0);
    }
});
