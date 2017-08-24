use point::Point;
use speed::Speed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
