use screen::{Screen, Pen};
use speed::Speed;
use point::Point;

pub struct IDEOutput;

impl IDEOutput {
    pub fn new() -> Self {
        IDEOutput
    }
}

impl Screen for IDEOutput {
    fn draw_line(&mut self, start: Point, end: Point, duration: Speed, pen: Pen) {
        unimplemented!();
    }
}
