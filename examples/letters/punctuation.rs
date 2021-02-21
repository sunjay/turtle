use std::f64::consts::PI;
use turtle::Turtle;

pub fn colon(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.backward(font_size / 4.0);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 100.0);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.left(90.0);
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 100.0);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
}

pub fn comma(turtle: &mut Turtle, font_size: f64) {
    turtle.right(90.0);
    turtle.pen_up();
    turtle.backward(font_size / 3.0);
    turtle.pen_down();
    for i in 0..50 {
        turtle.set_pen_size(3.0 - ((i as f64 + 1.0) / 25.5));
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.forward(font_size / 4.0);
    turtle.left(90.0);
}

pub fn dot(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.right(90.0);
    turtle.backward(font_size / 4.0);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 100.0);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
}

pub fn exclamation(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.right(90.0);
    turtle.backward(font_size / 3.0);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.set_pen_size(3.0);
    turtle.forward(font_size * 0.1);
    turtle.set_pen_size(1.0);
    turtle.pen_up();
    turtle.forward(font_size * 0.1);
    turtle.pen_down();
    turtle.forward(font_size * 0.8);
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn inverted_exclamation(turtle: &mut Turtle, font_size: f64) {
    turtle.set_pen_size(3.0);
    turtle.forward(font_size * 0.1);
    turtle.set_pen_size(1.0);
    turtle.pen_up();
    turtle.backward(font_size * 0.2);
    turtle.pen_down();
    turtle.backward(font_size * 0.8);
    turtle.pen_up();
    turtle.forward(font_size * 0.9);
    turtle.right(90.0);
    turtle.forward(font_size / 3.0);
    turtle.left(90.0);
}

pub fn inverted_question(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.backward(font_size);
    turtle.pen_down();
    turtle.right(90.0);
    for _ in 0..50 {
        turtle.backward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.left(90.0);
    turtle.backward(font_size * 0.3);
    turtle.pen_up();
    turtle.backward(font_size * 0.1);
    turtle.pen_down();
    turtle.set_pen_size(3.0);
    turtle.backward(font_size * 0.1);
    turtle.set_pen_size(1.0);
    turtle.left(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 4.0);
    turtle.left(90.0);
}

pub fn question(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_down();
    for _ in 0..50 {
        turtle.forward(PI / 200.0 * font_size);
        turtle.right(3.6);
    }
    turtle.left(90.0);
    turtle.forward(font_size * 0.3);
    turtle.pen_up();
    turtle.forward(font_size * 0.1);
    turtle.pen_down();
    turtle.set_pen_size(3.0);
    turtle.forward(font_size * 0.1);
    turtle.set_pen_size(1.0);
    turtle.left(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.left(90.0);
}

pub fn semicolon(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.backward(font_size / 4.0);
    turtle.pen_down();
    for _ in 0..100 {
        turtle.forward(PI / 100.0);
        turtle.left(3.6);
    }
    turtle.pen_up();
    turtle.left(90.0);
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.forward(font_size / 3.0);
    turtle.left(90.0);
    turtle.pen_down();
    comma(turtle, font_size);
}

pub fn space(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.left(90.0);
}
