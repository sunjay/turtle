use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..4 {
        turtle.forward(200.0);
        turtle.right(90.0);
    }
}
