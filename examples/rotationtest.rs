//! This program demonstrates rotations at different speeds

use std::time::Instant;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    let levels = 25;
    let test_area_width = 720.0;

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.left(90.0);
    turtle.forward(test_area_width / 2.0);
    turtle.right(180.0);
    turtle.pen_down();

    for i in 1..=levels {
        turtle.set_speed(i);
        let start = Instant::now();
        turtle.right(2.0 * 360.0);
        let elapsed = start.elapsed();
        println!("right {}: {} ms", i, elapsed.as_millis());

        turtle.pen_up();
        turtle.set_speed("instant");
        turtle.forward(test_area_width / levels as f64);
        turtle.pen_down();
    }

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.backward(test_area_width);
    turtle.pen_down();

    for i in 1..=levels {
        turtle.set_speed(i);
        let start = Instant::now();
        turtle.left(2.0 * 360.0);
        let elapsed = start.elapsed();
        println!("left {}: {} ms", i, elapsed.as_millis());

        turtle.pen_up();
        turtle.set_speed("instant");
        turtle.forward(test_area_width / levels as f64);
        turtle.pen_down();
    }
}
