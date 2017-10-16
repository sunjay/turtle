extern crate turtle;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_speed(10);

    turtle.pen_up();
    turtle.forward(150.0);
    turtle.pen_down();

    gear(&mut turtle);

    turtle.pen_up();
    turtle.forward(45.0);
    turtle.left(90.0);
    turtle.pen_down();

    circle(&mut turtle);

    turtle.hide();
}

fn gear(turtle: &mut Turtle) {
    // These variable names don't actually mean much because this was created by guessing and
    // then checking and the result doesn't exactly abide by the values as they are shown.
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

fn circle(turtle: &mut Turtle) {
    for _ in 0..360 {
        turtle.forward(1.5);
        turtle.right(1.0);
    }
}
