//! This example draws a crude version of the Rust logo.
//! The version is "crude" because it's all very approximate with plenty of guesswork.
//! If you can come up with a way to do this so that it exactly matches the logo, please
//! do submit it!
//! In any case, this is an excellent example of creatively using fills and some other
//! techniques to create a fairly complex drawing.

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_fill_color("black");

    turtle.begin_fill();
    turtle.pen_up();
    turtle.forward(130.0);
    turtle.pen_down();

    gear(&mut turtle);
    turtle.end_fill();

    turtle.pen_up();
    turtle.forward(45.0);
    turtle.left(90.0);
    turtle.pen_down();

    inner_gear(&mut turtle);

    letter(&mut turtle);

    turtle.hide();
}

fn gear(turtle: &mut Turtle) {
    // Much of this code was figured out by guessing and checking
    let teeth = 32.0;
    let tooth_size = 20.0;
    let angle = 360.0 / teeth;
    turtle.right(180.0 - angle * 6.0 / 2.0);
    for _ in 0..teeth as u64 {
        turtle.forward(tooth_size);
        turtle.left(180.0 - angle * 7.0);
        turtle.forward(tooth_size);
        turtle.right(180.0 - angle * 6.0);
    }

    turtle.right(angle * 6.0 / 2.0);
}

fn inner_gear(turtle: &mut Turtle) {
    inner_circle(turtle);
    inner_teeth(turtle);
}

fn inner_circle(turtle: &mut Turtle) {
    turtle.set_fill_color("white");
    turtle.begin_fill();
    for _ in 0..360 {
        turtle.forward(1.5);
        turtle.right(1.0);
    }
    turtle.end_fill();
    turtle.set_fill_color("black");
}

fn inner_teeth(turtle: &mut Turtle) {
    let tooth_size = 40.0;
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(10.0);
    turtle.right(90.0);
    turtle.pen_down();

    for _ in 0..5 {
        turtle.pen_up();
        turtle.forward(tooth_size / 2.0);
        turtle.pen_down();

        turtle.begin_fill();
        turtle.right(120.0);
        turtle.forward(tooth_size);
        turtle.right(120.0);
        turtle.forward(tooth_size);
        turtle.right(120.0);
        turtle.end_fill();

        turtle.pen_up();
        turtle.forward(tooth_size / 2.0);
        turtle.right(90.0);
        turtle.forward(5.0);
        turtle.left(90.0);
        turtle.pen_down();

        inner_tooth_circle(turtle);

        turtle.pen_up();
        turtle.left(90.0);
        turtle.forward(5.0);
        turtle.right(90.0);
        for _ in 0..360 / 5 / 2 {
            turtle.forward(3.41);
            turtle.right(2.0);
        }
        turtle.pen_down();
    }
}

fn inner_tooth_circle(turtle: &mut Turtle) {
    turtle.set_fill_color("white");
    turtle.begin_fill();
    for _ in 0..360 / 8 {
        turtle.forward(1.0);
        turtle.right(8.0);
    }
    turtle.end_fill();
    turtle.set_fill_color("black");
}

fn letter(turtle: &mut Turtle) {
    turtle.pen_up();
    turtle.right(180.0);
    turtle.pen_down();
    for _ in 0..(360 / 5 - 8) / 2 {
        turtle.forward(3.41);
        turtle.left(2.0);
    }
    // Trick for making the turtle face to the right
    let heading = turtle.heading();
    turtle.right(heading);

    turtle.set_pen_size(24.0);
    turtle.forward(110.0);
    for _ in 0..120 {
        turtle.forward(0.5);
        turtle.right(1.5);
    }

    turtle.forward(52.0);
    turtle.right(90.0);
    turtle.forward(35.0);
    turtle.backward(75.0);

    turtle.left(90.0);
    turtle.forward(65.0);
    turtle.backward(80.0);

    // Flatten tip of the bottom line of the letter "R"
    // This corrects for the rounded tip of the turtle's pen
    turtle.set_pen_size(5.0);

    turtle.backward(10.0);
    turtle.left(90.0);

    turtle.forward(9.5);
    turtle.right(90.0);
    turtle.forward(15.0);
    turtle.backward(15.0);
    turtle.left(90.0);

    turtle.backward(19.0);
    turtle.right(90.0);
    turtle.forward(15.0);
    turtle.backward(15.0);
    turtle.left(90.0);

    //resetting position after correction
    turtle.forward(9.5);
    turtle.right(90.0);

    turtle.set_pen_size(24.0);
    // end

    turtle.pen_up();
    turtle.backward(105.0);
    turtle.pen_down();

    turtle.forward(40.0);
    for _ in 0..80 {
        turtle.forward(0.5);
        turtle.right(1.0);
    }

    for _ in 0..30 {
        turtle.forward(0.5);
        turtle.left(0.5);
    }
}
