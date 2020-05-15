use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.right(90.0);
    turtle.set_pen_size(4.0);
    turtle.set_pen_color("yellow");

    for _ in 0..5 {
        turtle.forward(300.0);
        turtle.right(180.0 - (180.0 / 5.0));
    }
}
