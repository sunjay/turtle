extern crate turtle;
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.forward(100.0);
    turtle.turn_towards([0.0, 0.0]);
    turtle.forward(100.0);
    turtle.right(100.0);
    turtle.forward(100.0);
    turtle.turn_towards([::std::f64::INFINITY, ::std::f64::INFINITY]);
    turtle.forward(100.0);
}
