use piston_window::types;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
    Transparent,
}

impl From<Color> for types::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => [0., 0., 0., 255.],
            Color::White => [255., 255., 255., 255.],
            Color::Transparent => [0., 0., 0., 0.],
        }
    }
}
