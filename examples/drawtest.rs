extern crate piston_window;

use std::thread;
use std::sync::mpsc::{self, TryRecvError};

use piston_window::*;

enum Shape {
    Line {
        from: (f64, f64),
        to: (f64, f64),
        thickness: f64,
    },
}

enum Response {
    Ok,
}

fn main() {
    let (tx_drawing, rx_drawing) = mpsc::channel();
    let (tx_main, rx_main) = mpsc::channel();

    thread::spawn(move || {
        let mut window: PistonWindow = WindowSettings::new(
            "Hello Piston!", [800, 600]
        ).exit_on_esc(true).build().unwrap();
        let mut shapes = vec![];

        while let Some(e) = window.next() {
            match rx_drawing.try_recv() {
                Ok(shape) => {
                    shapes.push(shape);
                    tx_main.send(Response::Ok).unwrap();
                },
                Err(TryRecvError::Empty) => {}, // Do nothing
                Err(TryRecvError::Disconnected) => break, // Quit
            }

            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);

                for shape in &shapes {
                    match shape {
                        &Shape::Line {from, to, thickness} => {
                            line([0., 0., 0., 255.], thickness,
                                [from.0, from.1, to.0, to.1], c.transform, g);
                        },
                    }
                }
            });
        }
    });

    //TODO: Instead of unwrapping, we should quit since this means that the drawing
    //TODO: window has been closed (possibly due to an error on that thread)
    let draw = |shape: Shape| {
        tx_drawing.send(shape).unwrap();
        rx_main.recv().unwrap();
    };

    let center = (200., 200.);
    let radius = 100.;
    let thickness = 1.;
    let mut prev = (center.0 + radius, center.0);
    for i in 0..361 {
        let x = center.0 + radius * (i as f64).to_radians().cos();
        let y = center.1 + radius * (i as f64).to_radians().sin();
        draw(Shape::Line {
            from: (prev.0, prev.1),
            to: (x, y),
            thickness,
        });
        prev = (x, y);
    }
    draw(Shape::Line {
        from: (550., 150.),
        to: (350., 500.),
        thickness,
    });
    //handle.join().unwrap();
}
