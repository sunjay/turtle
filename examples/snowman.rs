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
    circle(turtle, 120.0);
    turtle.pen_up();
    sidestep_right(turtle, 120.0 * 2.0);
    turtle.pen_down();

    // Second circle is drawn in parts because we want the arms to be there too
    arc(turtle, 80.0, 120.0, 60);
    turtle.left(90.0);
    arm(turtle);
    turtle.right(90.0);
    arc(turtle, 80.0, 120.0, 60);
    turtle.left(90.0);
    arm(turtle);
    turtle.right(90.0);
    arc(turtle, 80.0, 120.0, 60);
    turtle.pen_up();
    sidestep_right(turtle, 80.0 * 2.0);
    turtle.pen_down();

    circle(turtle, 60.0);
    turtle.pen_up();
    sidestep_right(turtle, 60.0 * 2.0);
    turtle.pen_down();
}

fn arm(turtle: &mut Turtle) {
    let pen_color = turtle.pen_color();
    turtle.set_pen_color("brown");

    turtle.forward(150.0);

    turtle.pen_up();
    turtle.backward(40.0);
    turtle.left(25.0);
    turtle.pen_down();

    turtle.forward(60.0);

    turtle.pen_up();
    turtle.backward(60.0);
    turtle.right(25.0);
    turtle.backward(55.0);
    turtle.right(20.0);
    turtle.pen_down();

    turtle.forward(40.0);

    turtle.pen_up();
    turtle.backward(40.0);
    turtle.left(20.0);
    turtle.backward(55.0);
    turtle.pen_down();

    turtle.set_pen_color(pen_color);
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
    arc(turtle, radius, 360.0, 180)
}

fn arc(turtle: &mut Turtle, radius: f64, extent: f64, steps: u32) {
    let circumference = 2.0 * PI * radius;
    let distance = circumference * extent / 360.0;
    let step = distance / steps as f64;
    let rotation = extent / steps as f64;

    for _ in 0..steps {
        turtle.forward(step);
        turtle.right(rotation);
    }
}
