use std::time::Duration;

use point::Point;

#[derive(Serialize, Deserialize)]
pub enum Color {
    Black,
    White,
}

#[derive(Serialize, Deserialize)]
pub struct Pen {
    color: Color,
}

impl Default for Pen {
    fn default() -> Self {
        Pen {
            color: Color::Black,
        }
    }
}

/// This trait represents an output device for the turtle.
///
/// Each `Turtle` instance sends its commands using an instance
/// of this trait.
pub trait Screen {
    fn draw_line(&mut self, start: Point, end: Point, duration: Duration, pen: Pen);
}
