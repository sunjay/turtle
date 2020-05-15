// To run this example, use the command: cargo run --features unstable --example arrowkeys
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

use turtle::{Drawing, Event, event::{Key, PressedState}};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    println!("Turn using the left and right arrow keys");

    turtle.set_pen_color("red");
    turtle.set_pen_size(2.0);
    turtle.set_speed(4);
    loop {
        turtle.forward(1.0);

        while let Some(event) = drawing.poll_event() {
            use Key::{LeftArrow, RightArrow};
            match event {
                Event::Key(key, PressedState::Pressed) => match key {
                    LeftArrow => {
                        turtle.set_speed(8);
                        for _ in 0..20 {
                            turtle.forward(1.0);
                            turtle.left(4.5);
                        }
                        turtle.set_speed(4);
                    },
                    RightArrow => {
                        turtle.set_speed(8);
                        for _ in 0..20 {
                            turtle.forward(1.0);
                            turtle.right(4.5);
                        }
                        turtle.set_speed(4);
                    },
                    _ => {},
                },
                _ => {},
            }
        }
    }
}
