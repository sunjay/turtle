#[macro_use]
extern crate turtle;

use turtle::Color;

run_turtle!(|mut turtle| {

    turtle.set_background_color("light grey");
    turtle.set_speed(10);
    turtle.set_pen_size(2.0);
    for i in 0..290 {
        turtle.set_pen_color(Color {
            red: (i as f64 / 300.0 * 4.0) * 255.0 % 255.0,
            green: 255.0,
            blue: 255.0,
            alpha: 1.0,
        });
        turtle.forward(i as f64);

        turtle.right(60.0);
    }
});
