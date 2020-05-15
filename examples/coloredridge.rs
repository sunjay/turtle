use std::f64::consts::E;

use turtle::{rand::random, Color, Drawing};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    let amplitude = 100.0;
    let width = 800.0;
    let step = 2.0;
    let height_factor = 4.0;

    turtle.set_speed("instant");
    turtle.pen_up();
    turtle.right(90.0);
    turtle.backward(width / 2.0);
    turtle.pen_down();

    drawing.set_background_color("grey");

    turtle.set_speed("normal");
    for i in 0..(width / step) as i32 {
        let x = i as f64 * step;
        // y = e^(-x^2) translated and scaled by the width and amplitude
        // 200e^(-(1/200(x - 400))^2)
        let y = amplitude * E.powf(-(1.0 / (width / 4.0) * (x - width / 2.0)).powi(2));

        turtle.set_pen_color(random::<Color>().opaque());
        turtle.set_pen_size(y * height_factor);
        turtle.forward(step);
    }
}
