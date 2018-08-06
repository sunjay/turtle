//! This example shows how you can use color mixing and pen size to produce a smooth gradient

extern crate turtle;

use turtle::{Color, Turtle};

fn main() {
    let mut turtle = Turtle::new();
    turtle.drawing_mut().set_title("Gradients!");
    turtle.set_pen_size(200.0);

    turtle.pen_up();
    turtle.backward(250.0);
    turtle.pen_down();

    let red: Color = "red".into();
    let white: Color = "white".into();

    for i in 0..100 + 1 {
        turtle.set_pen_color(red.mix(white, i as f64 / 100.0));
        turtle.forward(5.0);
    }
}
