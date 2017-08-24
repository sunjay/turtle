use turtle::Distance;
use angle::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn origin() -> Self {
        Point {
            x: 0f64,
            y: 0f64,
        }
    }

    pub fn translate(self, direction: Direction, distance: Distance) -> Self {
        let angle = direction.to_radians();
        Point {
            x: self.x + distance * angle.cos(),
            y: self.y + distance * angle.sin(),
        }
    }
}
