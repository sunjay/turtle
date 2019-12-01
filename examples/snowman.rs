use std::f64::consts::PI;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.pen_up();
    turtle.backward(280.0);
    turtle.left(90.0);
    turtle.pen_down();

    body(&mut turtle);
    top_hat(&mut turtle);
    face(&mut turtle);

    turtle.hide();
}

fn body(turtle: &mut Turtle) {
    for &radius in &[120.0, 80.0, 60.0] {
        circle(turtle, radius);

        turtle.pen_up();
        sidestep_right(turtle, radius * 2.0);
        turtle.pen_down();
    }
}

fn top_hat(turtle: &mut Turtle) {
    turtle.set_fill_color("black");

    turtle.pen_up();
    sidestep_left(turtle, 10.0);
    turtle.pen_down();

    turtle.begin_fill();
    rectangle_bottom_center(turtle, 100.0, 10.0);
    turtle.end_fill();

    turtle.pen_up();
    sidestep_right(turtle, 10.0);
    turtle.pen_down();

    turtle.begin_fill();
    rectangle_bottom_center(turtle, 60.0, 40.0);
    turtle.end_fill();
}

fn face(turtle: &mut Turtle) {
    nose(turtle);
    eyes(turtle);
    smile(turtle);
}

fn nose(turtle: &mut Turtle) {
    // setup to draw nose
    turtle.pen_up();
    sidestep_left(turtle, 50.0);
    turtle.left(20.0);
    turtle.pen_down();

    // carrot nose
    turtle.set_fill_color("orange");
    turtle.begin_fill();
    turtle.forward(40.0);
    turtle.left(160.0);
    turtle.forward(40.0);
    turtle.end_fill();
}

fn eyes(turtle: &mut Turtle) {
    // setup to draw left eye
    turtle.pen_up();
    turtle.left(40.0);
    turtle.forward(25.0);
    turtle.left(140.0);
    turtle.pen_down();

    // left eye
    turtle.set_fill_color("black");
    turtle.begin_fill();
    circle(turtle, 5.0);
    turtle.end_fill();

    // setup for right eye
    turtle.pen_up();
    turtle.forward(40.0);
    turtle.pen_down();

    // right eye
    turtle.set_fill_color("black");
    turtle.begin_fill();
    circle(turtle, 5.0);
    turtle.end_fill();
}

fn smile(turtle: &mut Turtle) {
    // setup for start of smile
    turtle.pen_up();
    turtle.left(80.0);
    turtle.forward(35.0);
    turtle.left(80.0);
    turtle.pen_down();

    // draw arc for smile
    for _ in 0..25 {
        turtle.forward(2.0);
        turtle.left(2.0);
    }
}

/// Moves the turtle in the direction of its right side, retaining its original direction at the
/// end. Essentially a "sidestep"
fn sidestep_right(turtle: &mut Turtle, length: f64) {
    turtle.right(90.0);
    turtle.forward(length);
    turtle.left(90.0);
}

/// Moves the turtle in the direction of its left side, retaining its original direction at the
/// end. Essentially a "sidestep"
fn sidestep_left(turtle: &mut Turtle, length: f64) {
    turtle.left(90.0);
    turtle.forward(length);
    turtle.right(90.0);
}

/// Draws a rectangle starting at the bottom center point
fn rectangle_bottom_center(turtle: &mut Turtle, width: f64, height: f64) {
    turtle.forward(width/2.0);
    for &length in &[height, width, height] {
        turtle.right(90.0);
        turtle.forward(length);
    }
    turtle.right(90.0);
    turtle.forward(width/2.0);
}

fn circle(turtle: &mut Turtle, radius: f64) {
    arc(turtle, radius, 360.0)
}

fn arc(turtle: &mut Turtle, radius: f64, extent: f64) {
    let steps = 180.0;

    let circumference = 2.0 * PI * radius;
    let step = circumference / steps;
    let rotation = extent / steps;

    for _ in 0..steps as i32 {
        turtle.forward(step);
        turtle.right(rotation);
    }
}
