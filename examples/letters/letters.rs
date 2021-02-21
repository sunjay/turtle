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
    turtle.backward(font_size * 1.56);
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
    turtle.backward(font_size / 2.0);
    turtle.right(180.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.backward(font_size * 1.5);
    turtle.right(90.0);
}

pub fn c(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(0.5 * font_size);
    turtle.pen_down();
    for _ in 0..50 {
        turtle.backward(PI / 100.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
}

pub fn d(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    for _ in 0..50 {
        turtle.forward(PI / 100.0 * font_size);
        turtle.right(3.6);
    }
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
}

pub fn e(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.pen_down();
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn f(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.pen_down();
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(font_size * 1.5);
    turtle.left(90.0);
}

pub fn g(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(0.5 * font_size);
    turtle.right(90.0);
    turtle.forward(0.5 * font_size);
    turtle.pen_down();
    turtle.forward(0.5 * font_size);
    turtle.right(90.0);
    for i in 0..100 {
        if i == 87 {
            turtle.pen_up()
        }
        turtle.forward(PI / 100.0 * font_size);
        turtle.right(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn h(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.pen_down();
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.right(180.0);
    turtle.pen_up();
    turtle.forward(font_size * 1.5);
    turtle.left(90.0);
    turtle.backward(font_size / 2.0);
}

pub fn i(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size * 0.8);
    turtle.pen_up();
    turtle.forward(font_size * 0.1);
    turtle.pen_down();
    turtle.forward(font_size * 0.1);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn j(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.pen_down();
    turtle.left(180.0);
    for _ in 0..50 {
        turtle.forward(PI / 100.0 * font_size);
        turtle.left(3.6);
    }
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn k(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(font_size * 0.88);
    turtle.pen_down();
    turtle.left(30.0);
    turtle.backward(1.25_f64.sqrt() * font_size * 0.9);
    turtle.right(60.0);
    turtle.forward(1.25_f64.sqrt() * font_size * 0.9);
    turtle.left(30.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn l(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn m(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.left(30.0);
    turtle.backward(1.25_f64.sqrt() * font_size);
    turtle.right(60.0);
    turtle.forward(1.25_f64.sqrt() * font_size);
    turtle.left(30.0);
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn n(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.left(45.0);
    turtle.backward(2_f64.sqrt() * font_size);
    turtle.right(45.0);
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn o(turtle: &mut Turtle, font_size: f64) {
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(0.5 * font_size);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 100.0 * font_size);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.left(90.0);
}

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

pub fn w(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.pen_down();
    turtle.backward(font_size * 2.0 / 3.0);
    turtle.left(180.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.left(180.0);
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.left(3.6);
    }
    turtle.forward(font_size * 2.0 / 3.0);
    turtle.pen_up();
    turtle.left(180.0);
    turtle.forward(font_size);
    turtle.left(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}
