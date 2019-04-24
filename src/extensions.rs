//! Extension traits for various types

use crate::Point;

/// Conversions between different "interpretations" of coordinates.
pub trait ConvertScreenCoordinates {
    /// Transforms the given local coordinate into a point that can be drawn on the screen.
    ///
    /// Takes into account the direction of the axis and center when converting
    /// `local` from cartesian coordinates.
    ///
    /// Origin in window is the top left corner and the y-axis goes down instead of up.
    fn to_screen_coords(self, center: Self) -> Self;

    /// Transforms the given screen coordinates into local coordinates that
    /// are relative to the given center.
    ///
    /// Takes into account the difference in directions of the axis from screen to local
    fn to_local_coords(self, center: Self) -> Self;
}

impl ConvertScreenCoordinates for Point {
    fn to_screen_coords(self, center: Self) -> Self {
        Point {
            x: center.x + self.x,
            y: center.y - self.y,
        }
    }

    fn to_local_coords(self, center: Self) -> Self {
        Point {
            x: self.x - center.x,
            y: center.y - self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_screen_coords() {
        let center = Point { x: 300.0, y: 400.0 };

        let point = Point { x: 20.0, y: 30.0 };
        assert_eq!(point.to_screen_coords(center), Point { x: 320.0, y: 370.0 });
        assert_eq!(point.to_local_coords(center), Point { x: -280.0, y: 370.0 });
    }
}
