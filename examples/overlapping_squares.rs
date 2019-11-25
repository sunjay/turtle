use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for i in 0..10 {
        for _ in 0..4 {
            turtle.forward(100.0 + 10.0 * i as f64);
            turtle.right(90.0);
        }
    }
}
