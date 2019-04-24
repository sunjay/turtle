use turtle::{Turtle, Color};

fn main() {
    let mut turtle = Turtle::new();
    turtle.drawing_mut().set_background_color("light grey");

    for i in 0..720 {
        let i = i as f64;
        turtle.set_pen_color(Color::hsl(i % 360.0, 0.5, 0.5).with_alpha(1.0 - i / (360.0 * 2.5)));
        turtle.set_pen_size((i + 1.0) / 3.0);
        // Move forward three steps
        turtle.forward(6.0);
        turtle.backward(4.0);
        // Rotate to the right (clockwise) by 1 degree
        turtle.right(1.5);
    }
}
