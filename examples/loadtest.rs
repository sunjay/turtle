//! This example uses multiple turtles to try and do as much work concurrently as possible.
//! We are specifically loading the IPC connection here, so fewer turtles will be required to
//! make the drawing almost grind to a halt. A program that runs fewer turtle commands per
//! turtle would perform very differently.

// To run this example, use the command: cargo run --features unstable --example loadtest
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

use std::f64::consts::PI;
use std::sync::{Arc, Barrier};
use std::thread;

use turtle::{Drawing, Turtle, Color, rand::random};

/// This number is somewhat configurable, but the code doesn't robustly support all values
/// For example, non-perfect squares may not work.
const TURTLES: u32 = 20*20;

fn main() {
    let mut drawing = Drawing::new();

    let rows = (TURTLES as f64).sqrt();
    let cols = TURTLES as f64 / rows;

    // A "box" is an area of space on the screen given to a turtle to do its drawing
    let box_width = 20.0; // px
    let box_height = 20.0; // px

    drawing.set_size([(box_width * cols) as u32, (box_height * rows) as u32]);

    let turtles_ready = Arc::new(Barrier::new(TURTLES as usize));
    let turtles_done = Arc::new(Barrier::new(TURTLES as usize + 1));
    for i in 0..TURTLES {
        let mut turtle = drawing.add_turtle();

        let thread_turtles_ready = turtles_ready.clone();
        let thread_turtles_done = turtles_done.clone();
        thread::spawn(move || {
            let row = i / rows as u32;
            let col = i % rows as u32;

            // Get to the top of the row
            let y_offset = (row + 1) as f64 * box_height - rows * box_height / 2.0;
            // Get to the middle of the box
            let x_offset = col as f64 * box_width + box_width / 2.0 - cols * box_width / 2.0;

            turtle.pen_up();
            turtle.set_speed("instant");
            turtle.go_to((x_offset as f64, y_offset as f64));
            turtle.set_heading(0.0);
            turtle.set_speed("normal");
            turtle.pen_down();

            // Wait for all turtles to get into position
            thread_turtles_ready.wait();

            circles(&mut turtle);

            thread_turtles_done.wait();
        });
    }

    // If the main thread ends, the rest of the threads stop working, so it's best to wait
    turtles_done.wait();
}

fn circles(turtle: &mut Turtle) {
    let ncircles = 8;
    for i in 0..ncircles {
        circle(turtle, 8.0 * (ncircles - i) as f64 / ncircles as f64);
    }
}

fn circle(turtle: &mut Turtle, radius: f64) {
    let degrees = 180.0;

    let circumference = 2.0*PI*radius;
    let step = circumference / degrees;
    let rotation = 360.0 / degrees;

    turtle.set_pen_color(random::<Color>().opaque());
    turtle.set_fill_color(random::<Color>());

    turtle.begin_fill();
    for _ in 0..degrees as i32 {
        turtle.forward(step);
        turtle.right(rotation);
    }
    turtle.end_fill();
}
