use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.set_speed(5);
    turtle.set_pen_size(3.0);
    turtle.set_pen_color("red");

    turtle.pen_up();
    turtle.forward(-50.0);
    turtle.pen_down();

    turtle.set_fill_color("red");
    turtle.begin_fill();
    turtle.left(50.0);
    turtle.forward(111.65);
    turtle.set_speed(7);
    curve(&mut turtle);
    turtle.left(120.0);
    curve(&mut turtle);
    turtle.set_speed(5);
    turtle.forward(111.65);
    turtle.end_fill();

    end_loop(&mut turtle);

    turtle.drawing_mut().set_background_color("pink");
}

fn curve(turtle: &mut Turtle) {
    for _ in 0..100 {
        turtle.right(2.0);
        turtle.forward(2.0);
    }
}

fn end_loop(turtle: &mut Turtle) {
    turtle.forward(20.0);
    for _ in 0..10 {
        turtle.right(2.0);
        turtle.forward(3.0);
    }

    let speed = turtle.speed();
    turtle.set_speed(8);
    for _ in 0..60 {
        turtle.forward(1.5);
        turtle.backward(0.5);
        turtle.right(5.0);
    }
    turtle.set_speed(speed);

    turtle.right(15.0);
    turtle.forward(20.0);
    for _ in 0..10 {
        turtle.right(7.0);
        turtle.forward(5.0);
    }
}
