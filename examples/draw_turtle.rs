//! https://arkada38.github.io/2018/01/14/creating-a-turtle/
//!
//! This is inspired by a child's drawing of a turtle.
//! We are going to draw the turtle using arcs and straight lines.
//! To draw arcs, we use multiple Rust for-loops to create different tilt angles and
//! lengths. The more sophisticated the figure is, the more loops we need to make it.

use turtle::{Color, Drawing, Turtle, colors::BLACK};

const SIZE: f64 = 1.0;
const SHELL_COLOR: Color = Color {
    red: 62.0,
    green: 114.0,
    blue: 29.0,
    alpha: 1.0,
};
const BODY_COLOR: Color = Color {
    red: 119.0,
    green: 178.0,
    blue: 85.0,
    alpha: 1.0,
};
const EYE_COLOR: Color = BLACK;

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();
    drawing.set_size([250, 250]);

    turtle.set_speed(8);

    turtle.pen_up();
    turtle.set_x(-70.0);
    turtle.set_y(-20.0);

    draw_shell(&mut turtle);

    draw_tail(&mut turtle);

    turtle.pen_up();
    turtle.right(55.0);
    turtle.forward(SIZE * 10.0);
    turtle.right(55.0);

    // Back leg
    draw_leg(&mut turtle);

    turtle.right(86.5);
    turtle.forward(SIZE * 55.0);
    turtle.right(86.0);

    // Front leg
    draw_leg(&mut turtle);

    turtle.right(54.5);
    turtle.forward(SIZE * 8.0);

    draw_neck(&mut turtle);

    turtle.right(172.6);
    turtle.forward(SIZE * 40.0);
    turtle.right(110.0);

    draw_head(&mut turtle);

    turtle.left(128.0);
    turtle.forward(SIZE * 15.0);

    draw_eye(&mut turtle);

    turtle.set_fill_color(BODY_COLOR);

    turtle.left(175.0);
    turtle.forward(SIZE * 43.0);

    // Here we start to draw highlights on the shell.
    // We have 3 highlights: on the right, in the middle and on the left.

    draw_right_highlight(&mut turtle);

    turtle.right(18.0);
    turtle.forward(SIZE * 37.0);
    turtle.set_heading(180.0);

    draw_middle_highlight(&mut turtle);

    turtle.left(24.0);
    turtle.forward(SIZE * 36.0);
    turtle.set_heading(180.0);

    draw_left_highlight(&mut turtle);
}

fn draw_shell(turtle: &mut Turtle) {
    turtle.set_fill_color(SHELL_COLOR);
    turtle.begin_fill();

    for _ in 0..180 {
        turtle.forward(SIZE);
        turtle.right(1.0);
    }

    for _ in 0..90 {
        turtle.forward(SIZE / 3.0);
        turtle.right(1.0);
    }

    turtle.set_speed(5);
    let d = SIZE * 360.0 / std::f64::consts::PI;
    turtle.forward(d - d / 3.0);
    turtle.set_speed(10);

    for _ in 0..90 {
        turtle.forward(SIZE / 3.0);
        turtle.right(1.0);
    }

    turtle.end_fill();
}

fn draw_tail(turtle: &mut Turtle) {
    turtle.set_fill_color(BODY_COLOR);
    turtle.begin_fill();

    turtle.left(90.0);
    turtle.forward(SIZE);

    for _ in 0..45 {
        turtle.forward(SIZE / 3.0);
        turtle.left(1.0);
    }

    turtle.forward(SIZE * 3.0);

    for _ in 0..25 {
        turtle.forward(SIZE / 3.0);
        turtle.left(6.0);
    }

    turtle.forward(SIZE * 9.0);
    turtle.right(6.0);
    turtle.forward(SIZE * 8.0);

    turtle.end_fill();
}

fn draw_leg(turtle: &mut Turtle) {
    turtle.begin_fill();

    for _ in 0..15 {
        turtle.forward(SIZE);
        turtle.left(0.5);
    }

    for _ in 0..90 {
        turtle.forward(SIZE / 6.0);
        turtle.left(1.0);
    }

    turtle.forward(SIZE * 3.0);

    for _ in 0..90 {
        turtle.forward(SIZE / 6.0);
        turtle.left(1.0);
    }

    turtle.forward(SIZE * 14.5);

    turtle.end_fill();
}

fn draw_neck(turtle: &mut Turtle) {
    turtle.begin_fill();

    for _ in 0..15 {
        turtle.forward(SIZE * 3.0);
        turtle.left(1.5);
    }

    turtle.end_fill();
    turtle.begin_fill();

    turtle.left(100.0);
    turtle.forward(SIZE * 20.0);
    turtle.left(80.0);

    for _ in 0..4 {
        turtle.forward(SIZE * 3.7);
        turtle.left(1.5);
    }

    turtle.left(30.0);

    for _ in 0..27 {
        turtle.forward(SIZE);
        turtle.right(1.0);
    }

    turtle.end_fill();
}

// In our case the most difficult part of the turtle is head. It requires six loops to draw.
// There was a way to draw it as one circle but
// I've decided to make it look a like a real turtle's head.
fn draw_head(turtle: &mut Turtle) {
    turtle.begin_fill();

    for _ in 0..20 {
        turtle.forward(SIZE * 1.2);
        turtle.left(1.0);
    }

    for _ in 0..10 {
        turtle.forward(SIZE * 1.2);
        turtle.left(4.0);
    }

    for _ in 0..10 {
        turtle.forward(SIZE / 1.5);
        turtle.left(7.0);
    }

    for _ in 0..10 {
        turtle.forward(SIZE);
        turtle.left(2.0);
    }

    for _ in 0..50 {
        turtle.forward(SIZE / 2.5);
        turtle.left(1.0);
    }

    for _ in 0..30 {
        turtle.forward(SIZE / 3.0);
        turtle.left(1.8);
    }

    for _ in 0..10 {
        turtle.forward(SIZE / 1.5);
        turtle.left(1.8);
    }

    turtle.end_fill();
}

fn draw_eye(turtle: &mut Turtle) {
    turtle.set_fill_color(EYE_COLOR);
    turtle.begin_fill();
    for _ in 0..90 {
        turtle.forward(SIZE / 3.0);
        turtle.right(4.0);
    }
    turtle.end_fill();
}

fn draw_right_highlight(turtle: &mut Turtle) {
    turtle.begin_fill();

    for _ in 0..39 {
        turtle.forward(SIZE / 5.0);
        turtle.left(2.0);
    }

    turtle.forward(SIZE * 36.0);

    for _ in 0..90 {
        turtle.forward(SIZE / 2.5);
        turtle.left(2.0);
    }

    for _ in 0..42 {
        turtle.forward(SIZE);
        turtle.left(0.9);
    }

    for _ in 0..26 {
        turtle.forward(SIZE / 5.0);
        turtle.left(2.0);
    }

    turtle.end_fill();
}

fn draw_middle_highlight(turtle: &mut Turtle) {
    turtle.begin_fill();

    for _ in 0..40 {
        turtle.forward(SIZE / 4.0);
        turtle.left(2.0);
    }

    turtle.forward(SIZE * 47.0);
    turtle.left(8.5);

    for _ in 0..90 {
        turtle.forward(SIZE / 2.0);
        turtle.left(2.0);
    }
    turtle.forward(SIZE / 2.0);

    turtle.left(8.5);
    turtle.forward(SIZE * 47.0);

    for _ in 0..40 {
        turtle.left(2.0);
        turtle.forward(SIZE / 4.0);
    }

    turtle.end_fill();
}

fn draw_left_highlight(turtle: &mut Turtle) {
    turtle.begin_fill();

    for _ in 0..26 {
        turtle.forward(SIZE / 5.0);
        turtle.left(2.0);
    }

    for _ in 0..42 {
        turtle.forward(SIZE);
        turtle.left(0.9);
    }

    for _ in 0..90 {
        turtle.forward(SIZE / 2.5);
        turtle.left(2.0);
    }

    turtle.forward(SIZE * 36.0);

    for _ in 0..39 {
        turtle.forward(SIZE / 5.0);
        turtle.left(2.0);
    }

    turtle.end_fill();
}
