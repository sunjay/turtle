use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for i in 0..360 {
        // `i / 18` will produce 0 for i = 0 to 17, 1 for i = 18 to 35, 2 for i = 36 to 53,
        // 3 for i = 54 to 71, and so on.
        //
        // We then test if that number is even using `% 2`. This tells us whether we're on an even
        // or odd segment of the dashed line. For even segments, the turtle moves forwards, but no
        // line is drawn on the screen.
        if (i / 18) % 2 == 0 {
            turtle.pen_up();
        }

        turtle.forward(3.0);
        turtle.right(1.0);

        if (i / 18) % 2 == 0 {
            turtle.pen_down();
        }
    }
}
