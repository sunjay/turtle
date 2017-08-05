use point::Point;
use speed::Speed;

pub enum Color {
    Black,
    White,
}

pub struct Settings {
    color: Color,
    background: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            color: Color::Black,
            background: Color::White,
        }
    }
}

/// This trait represents an output device for the turtle.
///
/// Each `Turtle` instance sends its commands using an instance
/// of this trait.
pub trait Screen {
    fn draw_line(&mut self, start: Point, end: Point, duration: Speed, settings: Settings);
}
