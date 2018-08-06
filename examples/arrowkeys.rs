extern crate turtle;

use turtle::event::Key::{Left, Right};
use turtle::Event::KeyPressed;
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    println!("Turn using the left and right arrow keys");

    turtle.set_pen_color("red");
    turtle.set_pen_size(1.0);
    turtle.set_speed(4);
    loop {
        turtle.forward(1.0);

        while let Some(event) = turtle.drawing_mut().poll_event() {
            match event {
                KeyPressed(key) => match key {
                    Left => {
                        turtle.set_speed(8);
                        for _ in 0..20 {
                            turtle.forward(1.0);
                            turtle.left(4.5);
                        }
                        turtle.set_speed(4);
                    }
                    Right => {
                        turtle.set_speed(8);
                        for _ in 0..20 {
                            turtle.forward(1.0);
                            turtle.right(4.5);
                        }
                        turtle.set_speed(4);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
