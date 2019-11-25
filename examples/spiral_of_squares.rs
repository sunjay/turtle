use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..36 {
        for _ in 0..4 {
            turtle.forward(100.0);
            turtle.right(90.0);
        }

        turtle.left(10.0);
    }
}
