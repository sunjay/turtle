use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for i in 0..36 {
        for _ in 0..4 {
            turtle.forward(100.0);
            turtle.right(90.0 - 2.0 * i as f64);
        }

        // Using home() is one of the simplest ways to get the turtle back to the center after it
        // has finished drawing the unwravelled square. Another method that would work is to go
        // backwards along the unwravelled square.
        turtle.pen_up();
        turtle.home();
        turtle.pen_down();
        turtle.left(10.0 * i as f64);
    }
}
