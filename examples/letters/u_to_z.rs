use std::f64::consts::PI;
use turtle::Turtle;

pub fn u(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.pen_down();
    turtle.backward(font_size / 2.0);
    turtle.left(180.0);
    for _ in 0..50 {
        turtle.forward(PI / 100.0 * font_size);
        turtle.left(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.left(180.0);
    turtle.forward(font_size);
    turtle.left(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn v(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.left(30.0);
    turtle.pen_down();
    turtle.backward(1.25_f64.sqrt() * font_size);
    turtle.right(60.0);
    turtle.forward(1.25_f64.sqrt() * font_size);
    turtle.pen_up();
    turtle.left(30.0);
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn w(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.pen_down();
    turtle.left(20.0);
    turtle.backward(font_size * 1.1);
    turtle.right(40.0);
    turtle.forward(font_size / 2.0);
    turtle.left(40.0);
    turtle.backward(font_size / 2.0);
    turtle.right(40.0);
    turtle.forward(font_size * 1.1);
    turtle.pen_up();
    turtle.left(20.0);
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
}

pub fn x(turtle: &mut Turtle, font_size: f64) {
    turtle.right(45.0);
    turtle.forward(2.0_f64.sqrt() * font_size);
    turtle.pen_up();
    turtle.left(45.0);
    turtle.backward(font_size);
    turtle.left(45.0);
    turtle.pen_down();
    turtle.forward(2.0_f64.sqrt() * font_size);
    turtle.pen_up();
    turtle.backward(2.0_f64.sqrt() * font_size);
    turtle.right(135.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn y(turtle: &mut Turtle, font_size: f64) {
    turtle.right(30.0);
    turtle.forward(1.25 * font_size);
    turtle.pen_up();
    turtle.backward(1.25 * font_size / 2.0);
    turtle.left(60.0);
    turtle.pen_down();
    turtle.forward(1.25 * font_size / 2.0);
    turtle.pen_up();
    turtle.backward(1.25 * font_size);
    turtle.right(120.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn z(turtle: &mut Turtle, font_size: f64) {
    turtle.right(45.0);
    turtle.forward(2.0_f64.sqrt() * font_size);
    turtle.left(135.0);
    turtle.forward(font_size);
    turtle.left(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
    turtle.forward(font_size / 4.0);
    turtle.pen_down();
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.forward(font_size / 4.0);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.backward(font_size * 1.50);
    turtle.right(90.0);
}
