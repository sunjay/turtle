// To run this example, use the command: cargo run --features unstable --example followmouse
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

use turtle::Event::MouseMove;
use turtle::Drawing;

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    println!("Move your mouse around the window");

    turtle.set_pen_color("red");
    turtle.set_pen_size(2.0);
    turtle.set_speed(8);

    let mut target = [0.0, 0.0].into();
    loop {
        turtle.forward(1.0);

        turtle.set_speed("instant");
        turtle.turn_towards(target);
        turtle.set_speed(8);

        while let Some(MouseMove(next_target)) = drawing.poll_event() {
            target = next_target;
        }
    }
}
