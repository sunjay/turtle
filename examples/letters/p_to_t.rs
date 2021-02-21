use std::f64::consts::PI;
use turtle::Turtle;

pub fn p(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.backward(font_size * 1.25);
    turtle.right(90.0);
    turtle.backward(font_size / 2.0);
}

pub fn q(turtle: &mut Turtle, font_size: f64) {
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(0.5 * font_size);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 100.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(45.0);
    turtle.pen_down();
    turtle.backward(2.0_f64.sqrt() * font_size / 2.0);
    turtle.pen_up();
    turtle.left(45.0);
    turtle.backward(0.5 * font_size);
    turtle.right(90.0);
}

pub fn r(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 3.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.forward(font_size / 3.0);
    turtle.left(135.0);
    turtle.forward(1.25_f64.sqrt() * font_size * 0.625);
    turtle.right(135.0);
    turtle.pen_up();
    turtle.backward(font_size * 0.5);
    turtle.right(90.0);
}

pub fn s(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.pen_down();
    turtle.backward(font_size / 2.0);
    for _ in 0..50 {
        turtle.backward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.backward(font_size / 4.0);
    for _ in 0..50 {
        turtle.backward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.backward(font_size / 2.0);
    turtle.pen_up();
    turtle.forward(font_size * 1.25);
    turtle.left(90.0);
}

pub fn t(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(font_size);
    turtle.left(90.0);
}
