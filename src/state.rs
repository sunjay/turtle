use radians::Radians;
use {Point, Speed, Color, color};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingState {
    pub title: String,
    pub background: Color,
    pub center: Point,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            title: "Turtle".to_owned(),
            background: color::WHITE,
            center: [0.0, 0.0],
            width: 800,
            height: 600,
            maximized: false,
        }
    }
}
