#[macro_use]
extern crate turtle;

use std::f64::consts::PI;

use turtle::Turtle;

run_turtle!(|mut turtle| {

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.backward(250.0);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.set_speed(6);

    for &radius in [120.0, 80.0, 60.0].into_iter() {
        circle(&mut turtle, radius);

        turtle.set_speed("instant");
        turtle.pen_up();
        turtle.right(90.0);
        turtle.forward(radius * 2.0);
        turtle.left(90.0);
        turtle.pen_down();
        turtle.set_speed(6);
    }

    turtle.hide();
});

fn circle(turtle: &mut Turtle, radius: f64) {
    let degrees = 180.0;

    let circumference = 2.0*PI*radius;
    let step = circumference / degrees;
    let rotation = 360.0 / degrees;

    for _ in 0..degrees as i32 {
        turtle.forward(step);
        turtle.right(rotation);
    }
}
