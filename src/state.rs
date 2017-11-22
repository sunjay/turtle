use radians::Radians;
use types::Point;
use speed::Speed;
use color::{self, Color};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub start: Point,
    pub end: Point,
    pub pen: Pen,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<Point>,
    pub fill_color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pen {
    pub enabled: bool,
    pub thickness: f64,
    pub color: Color,
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            enabled: true,
            thickness: 1.0,
            color: color::BLACK,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TurtleState {
    pub pen: Pen,
    pub fill_color: Color,
    pub position: Point,
    pub heading: Radians,
    pub speed: Speed,
    pub visible: bool,
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            pen: Pen::default(),
            fill_color: color::BLACK,
            position: [0.0, 0.0],
            heading: Radians::from_degrees_value(90.0),
            speed: Speed::Five,
            visible: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawingState {
    pub background: Color,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            background: color::WHITE,
        }
    }
}
