use std::f64::consts::PI;

use serde::{Serialize, Deserialize};

use crate::radians::Radians;
use crate::{color, Color, Point, Speed};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingState {
    pub title: String,
    pub background: Color,
    pub center: Point,
    pub width: u32,
    pub height: u32,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
}

impl DrawingState {
    pub const DEFAULT_TITLE: &'static str = "Turtle";
    pub const DEFAULT_BACKGROUND: Color = color::WHITE;
    pub const DEFAULT_CENTER: Point = Point::origin();
    pub const DEFAULT_WIDTH: u32 = 800;
    pub const DEFAULT_HEIGHT: u32 = 600;
    pub const DEFAULT_IS_MAXIMIZED: bool = false;
    pub const DEFAULT_IS_FULLSCREEN: bool = false;
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            title: Self::DEFAULT_TITLE.to_owned(),
            background: Self::DEFAULT_BACKGROUND,
            center: Self::DEFAULT_CENTER,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            is_maximized: Self::DEFAULT_IS_MAXIMIZED,
            is_fullscreen: Self::DEFAULT_IS_FULLSCREEN,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurtleState {
    pub pen: Pen,
    pub fill_color: Color,
    pub is_filling: bool,
    pub position: Point,
    pub heading: Radians,
    pub speed: Speed,
    pub is_visible: bool,
}

impl TurtleState {
    pub const DEFAULT_FILL_COLOR: Color = color::BLACK;
    pub const DEFAULT_IS_FILLING: bool = false;
    pub const DEFAULT_POSITION: Point = Point::origin();
    pub const DEFAULT_HEADING: Radians = Radians::from_radians_value(PI / 2.0);
    pub const DEFAULT_IS_VISIBLE: bool = true;
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            pen: Pen::default(),
            fill_color: Self::DEFAULT_FILL_COLOR,
            is_filling: Self::DEFAULT_IS_FILLING,
            position: Self::DEFAULT_POSITION,
            heading: Self::DEFAULT_HEADING,
            speed: Speed::default(),
            is_visible: Self::DEFAULT_IS_VISIBLE,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pen {
    pub is_enabled: bool,
    pub thickness: f64,
    pub color: Color,
}

impl Pen {
    pub const DEFAULT_IS_ENABLED: bool = true;
    pub const DEFAULT_THICKNESS: f64 = 1.0;
    pub const DEFAULT_COLOR: Color = color::BLACK;
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            is_enabled: Self::DEFAULT_IS_ENABLED,
            thickness: Self::DEFAULT_THICKNESS,
            color: Self::DEFAULT_COLOR,
        }
    }
}
