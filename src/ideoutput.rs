use std::io;
use std::time::Duration;

use serde_json;

use screen::{Screen, Pen};
use point::Point;

#[derive(Serialize, Deserialize)]
pub enum IDEResponse {
    Complete,
}

#[derive(Serialize, Deserialize)]
pub enum IDEMessage {
    DrawLine {
        start: Point,
        end: Point,
        duration: Duration,
        pen: Pen,
    },
}

pub struct IDEOutput;

impl IDEOutput {
    pub fn new() -> Self {
        IDEOutput
    }

    pub fn send(&self, message: IDEMessage) {
        let message = serde_json::to_string(&message).unwrap();
        println!("{}", message);
        self.wait_for_completion();
    }

    pub fn wait_for_completion(&self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let message: IDEResponse = serde_json::from_str(&input)
            .expect("Invalid message received from calling process");
        match message {
            IDEResponse::Complete => {},
        }
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
        self.send(IDEMessage::DrawLine {start, end, duration, pen});
    }
}
