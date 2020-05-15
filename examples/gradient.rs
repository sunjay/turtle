//! This example shows how you can use color mixing and pen size to produce a smooth gradient

use turtle::{Color, Drawing};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();
    drawing.set_title("Gradients!");
    turtle.set_pen_size(400.0);

    turtle.pen_up();
    turtle.backward(200.0);
    turtle.pen_down();

    let red: Color = "red".into();
    let white: Color = "white".into();

    for i in 0..100 + 1 {
        turtle.set_pen_color(red.mix(white, i as f64 / 100.0));
        turtle.forward(4.0);
    }
}
