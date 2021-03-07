use std::f64::consts::PI;
use turtle::Turtle;

pub fn acutte(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size * 1.5);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(45.0);
    turtle.pen_down();
    for i in 1..=10 {
        turtle.forward(font_size / 20.0);
        turtle.set_pen_size(font_size * i as f64 / 30.0);
    }
    turtle.set_pen_size(1.0);
    turtle.pen_up();
    turtle.backward(font_size / 3.0);
    turtle.left(45.0);
    turtle.backward(font_size * 1.5);
    turtle.right(90.0);
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
}

pub fn apostrophe(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size * 0.75);
    turtle.pen_down();
    for i in 1..=10 {
        turtle.forward(font_size / 20.0);
        turtle.set_pen_size(font_size * i as f64 / 40.0);
    }
    turtle.set_pen_size(1.0);
    turtle.pen_up();
    turtle.backward(font_size / 3.0);
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
}

pub fn tilde(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size * 1.5);
    turtle.pen_down();
    turtle.left(180.0);
    for _ in 0..50 {
        turtle.backward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    for _ in 0..50 {
        turtle.backward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.left(180.0);
    turtle.backward(font_size * 1.5);
    turtle.right(90.0);
    turtle.forward(font_size * 0.5);
    turtle.left(90.0);
    turtle.pen_down();
}

pub fn with_title(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(1.5 * font_size);
    turtle.right(90.0);
    turtle.pen_down();
}

pub fn with_accute(turtle: &mut Turtle, font_size: f64) {
    turtle.left(90.0);
    turtle.forward(font_size);
    turtle.pen_down();
    turtle.right(90.0);
}
