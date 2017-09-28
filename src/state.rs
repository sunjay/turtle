use radians::Radians;
use {Point, Speed, Color};

#[derive(Debug, Clone)]
pub struct Pen {
    pub enabled: bool,
    pub thickness: f64,
    pub color: Color,
}

pub struct TurtleState {
    pub position: Point,
    pub heading: Radians,
    pub speed: Speed,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub start: Point,
    pub end: Point,
    pub pen: Pen,
}

pub struct DrawingState {
    pub pen: Pen,
    pub background: Color,
}
