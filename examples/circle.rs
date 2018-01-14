#[macro_use]
extern crate turtle;

run_turtle!(|mut turtle| {
    for _ in 0..360 {
        // Move forward three steps
        turtle.forward(3.0);
        // Rotate to the right (clockwise) by 1 degree
        turtle.right(1.0);
    }
});
