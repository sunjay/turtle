use std::f64::consts::PI;
use turtle::Turtle;

pub fn a(turtle: &mut Turtle, font_size: f64) {
    turtle.right(30.0);
    turtle.forward(1.25_f64.sqrt() * font_size);
    turtle.right(120.0);
    turtle.forward(1.25_f64.sqrt() * font_size);
    turtle.pen_up();
    turtle.backward(1.25_f64.sqrt() * font_size / 2.0);
    turtle.right(120.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.56);
    turtle.pen_up();
    turtle.backward(font_size * 1.20);
    turtle.right(90.0);
    turtle.backward(font_size / 2.0);
}

pub fn b(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(180.0);
    turtle.pen_down();
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
}

pub fn c(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size * 0.70);
    turtle.pen_down();
    turtle.backward(font_size / 5.0);
    for _ in 0..50 {
        turtle.backward(PI / 100.0 * font_size);
        turtle.left(3.6);
    }
    turtle.backward(font_size / 5.0);
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
}

pub fn d(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 5.0);
    for _ in 0..50 {
        turtle.forward(PI / 100.0 * font_size);
        turtle.right(3.6);
    }
    turtle.forward(font_size / 5.0);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
}
