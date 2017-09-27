extern crate turtleide;

use turtleide::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..360 {
        turtle.forward(3.0);
        turtle.right(1.0);
    }
}
