use std::f64::consts::PI;
use turtle::Turtle;

pub fn e(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size * 0.75);
    turtle.right(90.0);
    turtle.pen_up();
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.75);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.75);
    turtle.pen_up();
    turtle.forward(font_size * 0.25);
    turtle.left(90.0);
}

pub fn f(turtle: &mut Turtle, font_size: f64) {
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.forward(font_size * 0.75);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.75);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.left(90.0);
}

pub fn g(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.right(45.0);
    turtle.forward(font_size * 2_f64.sqrt() / 2.0);
    turtle.right(45.0);
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
    turtle.forward(font_size * 0.75);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.backward(font_size / 2.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.75);
    turtle.right(180.0);
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.left(90.0);
    turtle.backward(font_size / 2.0);
}

pub fn i(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.75);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size * 0.75);
    turtle.pen_up();
    turtle.backward(font_size * 0.375);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.forward(font_size);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(font_size * 0.625);
    turtle.left(90.0);
    turtle.backward(font_size);
}
