use piston_window::types;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

pub const TRANSPARENT: Color = Color {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0};
pub const BLACK: Color = Color {red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0};
pub const WHITE: Color = Color {red: 255.0, green: 255.0, blue: 255.0, alpha: 1.0};

impl From<Color> for types::Color {
    fn from(color: Color) -> Self {
        [color.red as f32, color.green as f32, color.blue as f32, color.alpha as f32]
    }
}
