#[macro_use]
extern crate turtle;

use turtle::{Color, random};

run_turtle!(|mut turtle| {

    turtle.set_speed(8);
    turtle.set_pen_size(2.0);
    for i in 0..300 {
        turtle.set_background_color(random::<Color>().opaque());

        turtle.set_pen_color(random::<Color>().opaque());
        turtle.forward(i as f64);

        turtle.right(60.0);
    }
});
