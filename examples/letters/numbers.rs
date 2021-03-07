use std::f64::consts::PI;
use turtle::Turtle;

pub fn zero(turtle: &mut Turtle, font_size: f64) {
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(0.5 * font_size);
    turtle.pen_down();
    for _ in 0..25 {
        turtle.forward(PI / 133.0 * font_size);
        turtle.left(3.6);
    }
    turtle.forward(font_size / 3.0);
    for _ in 0..50 {
        turtle.forward(PI / 133.0 * font_size);
        turtle.left(3.6);
    }
    turtle.forward(font_size / 3.0);
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(font_size * 0.4);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.forward(font_size / 3.0);
    turtle.pen_up();
    turtle.backward(font_size / 3.0);
    turtle.left(90.0);
    turtle.forward(font_size * 0.4);
    turtle.left(90.0);
    turtle.pen_down();
    for _ in 0..25 {
        turtle.forward(PI / 133.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.left(90.0);
}

pub fn one(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.right(45.0);
    turtle.pen_down();
    turtle.forward(font_size * 2_f64.sqrt() / 2.0);
    turtle.left(45.0);
    turtle.backward(font_size);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
}

pub fn two(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 1.4);
    turtle.pen_down();
    for _ in 0..125 {
        turtle.forward(0.01 * font_size);
        turtle.right(1.8);
    }
    turtle.forward(font_size * 2_f64.sqrt() / 2.0);
    turtle.right(45.0);
    turtle.backward(font_size / 2.0);
    turtle.pen_up();
    turtle.backward(font_size * 0.25);
    turtle.right(90.0);
}

pub fn three(turtle: &mut Turtle, font_size: f64) {
    turtle.right(90.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.left(180.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(font_size);
    turtle.left(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn four(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.right(45.0);
    turtle.pen_down();
    turtle.forward(font_size * 2_f64.sqrt() / 2.0);
    turtle.left(45.0);
    turtle.backward(font_size);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.75);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.backward(font_size / 2.0);
}

pub fn five(turtle: &mut Turtle, font_size: f64) {
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.forward(font_size * 0.8);
    turtle.pen_up();
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
    turtle.backward(font_size);
}

pub fn six(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    for _ in 0..25 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_down();
    turtle.backward(font_size / 2.0);
    for _ in 0..50 {
        turtle.backward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.pen_up();
    turtle.backward(font_size * 0.75);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn seven(turtle: &mut Turtle, font_size: f64) {
    turtle.right(45.0);
    turtle.forward(font_size * 2_f64.sqrt());
    turtle.right(45.0);
    turtle.backward(font_size);
    turtle.pen_up();
    turtle.forward(font_size * 1.25);
    turtle.left(90.0);
    turtle.backward(font_size);
}

pub fn eight(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    for _ in 0..100 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.backward(font_size / 2.0);
}

pub fn nine(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.backward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    for _ in 0..25 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.pen_down();
    turtle.forward(font_size / 2.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.left(90.0);
    turtle.backward(font_size / 4.0);
}
