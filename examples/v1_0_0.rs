use turtle::{Color, Drawing, Turtle};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();
    drawing.set_title("Version 1.0.0!!!");

    drawing.set_background_color("#FFEB3B");
    turtle.set_pen_size(10.0);

    turtle.set_speed("instant");
    turtle.pen_up();
    turtle.go_to((350.0, 178.0));
    turtle.pen_down();

    bg_lines(&mut turtle);

    turtle.pen_up();
    turtle.go_to((-270.0, -200.0));
    turtle.set_heading(90.0);
    turtle.pen_down();

    turtle.set_speed("normal");
    turtle.set_pen_color("#2196F3");
    turtle.set_fill_color(Color::from("#00E5FF").with_alpha(0.75));

    one(&mut turtle);

    turtle.set_speed(25);

    turtle.pen_up();
    turtle.left(90.0);
    turtle.backward(50.0);
    turtle.pen_down();

    small_circle(&mut turtle);

    turtle.pen_up();
    turtle.backward(150.0);
    turtle.pen_down();

    zero(&mut turtle);

    turtle.pen_up();
    turtle.backward(150.0);
    turtle.pen_down();

    small_circle(&mut turtle);

    turtle.pen_up();
    turtle.backward(150.0);
    turtle.pen_down();

    zero(&mut turtle);
}

fn bg_lines(turtle: &mut Turtle) {
    turtle.set_pen_color("#76FF03");
    turtle.set_heading(165.0);
    turtle.forward(280.0);

    turtle.left(147.0);
    turtle.forward(347.0);

    turtle.right(158.0);
    turtle.forward(547.0);

    turtle.left(138.0);
    turtle.forward(539.0);

    turtle.right(168.0);
    turtle.forward(477.0);

    turtle.left(154.0);
    turtle.forward(377.0);

    turtle.right(158.0);
    turtle.forward(329.0);
}

fn small_circle(turtle: &mut Turtle) {
    turtle.begin_fill();
    for _ in 0..90 {
        turtle.forward(1.0);
        turtle.right(4.0);
    }
    turtle.end_fill();
}

fn one(turtle: &mut Turtle) {
    turtle.begin_fill();
    for _ in 0..2 {
        turtle.forward(420.0);
        turtle.left(90.0);
        turtle.forward(50.0);
        turtle.left(90.0);
    }
    turtle.end_fill();
}

fn zero(turtle: &mut Turtle) {
    turtle.begin_fill();
    for _ in 0..2 {
        arc_right(turtle);
        arc_forward(turtle);
    }
    turtle.end_fill();
}

fn arc_right(turtle: &mut Turtle) {
    // Draw an arc that moves right faster than it moves forward
    for i in 0..90 {
        turtle.forward(3.0);
        turtle.right((90 - i) as f64 / 45.0);
    }
}

fn arc_forward(turtle: &mut Turtle) {
    // Draw an arc that moves forward faster than it moves right
    for i in 0..90 {
        turtle.forward(3.0);
        turtle.right(i as f64 / 45.0);
    }
}
