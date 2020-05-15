use turtle::{Drawing, Color};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();
    drawing.set_background_color("light grey");

    for i in 0..720 {
        let i = i as f64;
        turtle.set_pen_color(Color::hsl(i % 360.0, 0.5, 0.5).with_alpha(1.0 - i / (360.0 * 2.5)));
        turtle.set_pen_size((i + 1.0) / 1.5);
        // Move forward three steps
        turtle.forward(2.0);
        // Rotate to the right (clockwise) by 1 degree
        turtle.right(1.5);
    }
}
