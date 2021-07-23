use turtle::*;

fn main() {
    let mut turtle = Turtle::new();

    turtle.pen_up();
    turtle.go_to((-100.0, -100.0));
    turtle.pen_down();
    turtle.begin_fill();
    turtle.set_pen_color("green");
    turtle.set_fill_color("green");
    for _ in 0..4 {
        turtle.forward(200.0);
        turtle.right(90.0);
    }
    turtle.end_fill();
}
