use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for i in 0..36 {
        for _ in 0..4 {
            turtle.forward(100.0 + i as f64 * 5.0);
            turtle.right(90.0);
        }

        turtle.left(10.0);
    }
}
