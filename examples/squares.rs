use turtle::{Drawing, Turtle, Color};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();
    drawing.set_background_color("pink");

    for i in 0..36 {
        let base_color: Color = if i % 2 == 0 {
            "red".into()
        } else {
            "white".into()
        };
        turtle.set_fill_color(base_color.with_alpha(1.0 - i as f64 / 54.0));
        turtle.begin_fill();
        square(&mut turtle);
        turtle.end_fill();
        turtle.right(10.0);
    }
}

fn square(turtle: &mut Turtle) {
    for _ in 0..4 {
        turtle.forward(200.0);
        turtle.right(90.0);
    }
}
