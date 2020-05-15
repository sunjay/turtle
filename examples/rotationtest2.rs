//! This program demonstrates rotations at different speeds

use std::time::Instant;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    let levels = 25;
    let test_area_width = 740.0;

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.left(90.0);
    turtle.forward(test_area_width / 2.0);
    turtle.right(180.0);
    turtle.pen_down();

    turtle.hide();
    for i in 1..=levels {
        turtle.set_speed(i);
        let start = Instant::now();
        circle(&mut turtle);
        let elapsed = start.elapsed();
        println!("circle {}: {} ms", i, elapsed.as_millis());

        turtle.pen_up();
        turtle.set_speed("instant");
        turtle.forward(test_area_width / levels as f64);
        turtle.pen_down();
    }
}

fn circle(turtle: &mut Turtle) {
    for _ in 0..90 {
        turtle.forward(1.0);
        turtle.left(4.0);
    }
}
