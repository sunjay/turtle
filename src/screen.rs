use point::Point;
use speed::Speed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Used for when the pen is up
    Transparent,
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pen {
    turtle_visible: bool,
    color: Color,
}

impl Pen {
    pub fn as_transparent(&self) -> Pen {
        Pen {
            color: Color::Transparent,
            ..*self
        }
    }
}

impl Default for Pen {
    fn default() -> Self {
        Pen {
            turtle_visible: true,
            color: Color::Black,
        }
    }
}

pub struct Screen;

impl Default for Screen {
    fn default() -> Self {
        Screen
    }
}

impl Screen {
    pub fn draw_line(&mut self, start: Point, end: Point, speed: Speed, pen: &Pen) -> Point {
        unimplemented!();
    }
}
