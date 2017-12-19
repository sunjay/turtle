#[macro_use]
extern crate turtle;

use turtle::{Color};

run_turtle!(|mut turtle| {

    turtle.set_speed(8);
    turtle.set_pen_size(2.0);
    for i in 0..300 {
        let bg_color = turtle.random::<Color>().opaque();
        turtle.set_background_color(bg_color);

        let pen_color = turtle.random::<Color>().opaque();
        turtle.set_pen_color(pen_color);
        turtle.forward(i as f64);

        turtle.right(60.0);
    }
});
