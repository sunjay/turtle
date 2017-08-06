use std::io;
use std::time::Duration;

use screen::{Screen, Pen};
use point::Point;

pub struct IDEOutput;

impl IDEOutput {
    pub fn new() -> Self {
        IDEOutput
    }

    pub fn wait_for_completion(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        if input == "Complete" {
            return;
        }
        panic!("No completion message recieved from calling process after command was completed")
    }
}

impl Screen for IDEOutput {
    fn draw_line(
        &mut self,
        start: Point,
        end: Point,
        duration: Duration,
        pen: Pen,
    ) {
        unimplemented!();
    }
}
