use turtle::Event::MouseMove;
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    println!("Move your mouse around the window");

    turtle.set_pen_color("red");
    turtle.set_pen_size(1.0);
    turtle.set_speed(8);

    let mut target = [0.0, 0.0];
    loop {
        turtle.forward(1.0);

        turtle.set_speed("instant");
        turtle.turn_towards(target);
        turtle.set_speed(8);

        while let Some(MouseMove { x, y }) = turtle.drawing_mut().poll_event() {
            target = [x, y];
        }
    }
}
