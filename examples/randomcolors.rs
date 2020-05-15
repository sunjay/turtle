use turtle::{rand::random, Color, Drawing};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    turtle.set_speed(8);
    turtle.set_speed(20);
    turtle.set_pen_size(4.0);
    for i in 0..300 {
        drawing.set_background_color(random::<Color>().opaque());

        turtle.set_pen_color(random::<Color>().opaque());
        turtle.forward(i as f64);

        turtle.right(60.0);
    }
}
