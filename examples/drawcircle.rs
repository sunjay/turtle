extern crate turtleide;

use turtleide::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..360 {
        turtle.forward(10);
        turtle.right(1f64);
    }
}
