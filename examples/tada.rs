extern crate turtle;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.pen_up();
    go_to(&mut turtle, (-250.0, -200.0));
    turtle.set_heading(60.0);
    turtle.pen_down();

    turtle.set_fill_color("#f44336");
    turtle.set_pen_color("#212121");
    turtle.set_pen_size(5.0);

    turtle.begin_fill();
    turtle.forward(400.0);
    turtle.right(110.0);
    turtle.forward(350.0);
    turtle.right(110.0);
    turtle.forward(400.0);
    turtle.right(70.0);
    turtle.forward(80.0);
    turtle.end_fill();

    turtle.pen_up();
    go_to(&mut turtle, (40.0, 110.0));
    turtle.set_heading(60.0);
    turtle.pen_down();

    turtle.set_pen_color("#9C27B0");
    streamer(&mut turtle);

    turtle.pen_up();
    go_to(&mut turtle, (100.0, 45.0));
    turtle.set_heading(35.0);
    turtle.pen_down();

    turtle.set_pen_color("#4CAF50");
    streamer(&mut turtle);

    turtle.pen_up();
    go_to(&mut turtle, (150.0, -40.0));
    turtle.set_heading(15.0);
    turtle.pen_down();

    turtle.set_pen_color("#2196F3");
    streamer(&mut turtle);
}

fn streamer(turtle: &mut Turtle) {
    for i in 1..25 {
        turtle.set_pen_size(i as f64);
        turtle.forward(8.0);
    }
}

fn go_to(turtle: &mut Turtle, pt: (f64, f64)) {
    turtle.turn_towards(pt);
    turtle.go_to(pt);
}
