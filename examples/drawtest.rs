extern crate piston_window;

use canvas::{Canvas, Shape};

fn main() {
    let canvas = Canvas::new();

    let center = (200., 200.);
    let radius = 100.;
    let thickness = 1.;
    let mut prev = (center.0 + radius, center.0);
    for i in 0..361 {
        let x = center.0 + radius * (i as f64).to_radians().cos();
        let y = center.1 + radius * (i as f64).to_radians().sin();
        canvas.draw(Shape::Line {
            from: (prev.0, prev.1),
            to: (x, y),
            thickness,
        });
        prev = (x, y);
    }
    canvas.draw(Shape::Line {
        from: (550., 150.),
        to: (350., 500.),
        thickness,
    });
}

mod canvas {
    use std::thread;
    use std::process;
    use std::sync::mpsc::{self, TryRecvError};

    use piston_window::*;

    pub enum Shape {
        Line {
            from: (f64, f64),
            to: (f64, f64),
            thickness: f64,
        },
    }

    pub type Response = Result<(), ()>;

    pub struct Canvas {
        thread_handle: Option<thread::JoinHandle<()>>,
        transmitter: mpsc::Sender<Shape>,
        receiver: mpsc::Receiver<Response>,
    }

    impl Canvas {
        pub fn new() -> Canvas {
            let (tx_drawing, rx_drawing) = mpsc::channel();
            let (tx_main, rx_main) = mpsc::channel();

            let handle = thread::spawn(move || {
                let mut window: PistonWindow = WindowSettings::new(
                    "Turtle IDE", [800, 600]
                ).exit_on_esc(true).build().unwrap();
                let mut shapes = vec![];

                while let Some(e) = window.next() {
                    match rx_drawing.try_recv() {
                        Ok(shape) => {
                            shapes.push(shape);
                            tx_main.send(Ok(())).unwrap();
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

            Canvas {
                thread_handle: Some(handle),
                transmitter: tx_drawing,
                receiver: rx_main,
            }
        }

        pub fn draw(&self, shape: Shape) {
            let result = self.transmitter.send(shape).map_err(|_| ());
            let result = result.and_then(|_| {
                // Wait for the drawing animation to complete
                self.receiver.recv().map_err(|_| ())
            });
            match result {
                Ok(_) => {},
                Err(_) => {
                    // The connection has been closed so the window was closed
                    // or an error occurred on that thread
                    process::exit(0);
                },
            }
        }
    }

    impl Drop for Canvas {
        fn drop(&mut self) {
            if let Some(handle) = self.thread_handle.take() {
                handle.join().unwrap();
            }
        }
    }
}
