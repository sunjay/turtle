extern crate turtle;

use turtle::*;

fn draw(turtle: &mut Turtle, side: f64, deep: usize) {
    let next_side = side / f64::sqrt(2.0);

    for _ in 0..4 {
        let position = turtle.position();
        turtle.set_pen_color(get_color(position));
        turtle.right(90.0);
        turtle.forward(side);
    }

    if deep > 0 {
        turtle.pen_up();
        turtle.left(45.0);
        turtle.forward(next_side);
        let position = turtle.position();
        turtle.pen_down();

        let heading = turtle.heading();

        draw(turtle, next_side, deep - 1);
        turtle.set_heading(heading);

        turtle.pen_up();
        turtle.go_to(position);
        turtle.right(135.0);
        turtle.forward(side);
        turtle.left(45.0);
        turtle.forward(next_side);
        turtle.pen_down();

        let heading = turtle.heading();

        draw(turtle, next_side, deep - 1);
        turtle.set_heading(heading);
    }
}

fn get_color(position: [f64;2]) -> Color {
    Color{
        red: 130.0 + f64::abs(position[0] % 75.0),
        green: 150.0 + f64::abs(position[1] % 55.0),
        blue: 210.0 + f64::abs(position[1] % 25.0),
        alpha: 0.8
    }
}

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed(Speed::Ten);
    let side = 100.0;

    turtle.pen_up();
    turtle.backward(1.5 * side);
    turtle.pen_down();

    draw(&mut turtle, side, 6);
}
