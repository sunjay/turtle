use turtle::Event::MouseMove;
use turtle::Drawing;

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

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

        while let Some(MouseMove { x, y }) = drawing.poll_event() {
            target = [x, y];
        }
    }
}
