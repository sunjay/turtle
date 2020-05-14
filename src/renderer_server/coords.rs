//! Conversions between coordinate systems
//!
//! # Coordinate Systems
//!
//! logical or "world" coordinates (cartesian coordinates)
//!   * origin is in the center of the framebuffer and can be offset by drawing `center`
//!   * x is left to right
//!   * y is bottom to top
//!
//! screen coordinates
//!   * origin is the top-left corner of the framebuffer
//!   * x is left to right
//!   * y is top to bottom

use pathfinder_geometry::vector::{Vector2F, vec2f};

use crate::Point;

/// A point in the screen corodinate system
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenPoint {
    pub x: f64,
    pub y: f64,
}

/// A `Vector2F` is considered to be in screen coordinates as this is the type used in the renderer
impl From<Vector2F> for ScreenPoint {
    fn from(vec: Vector2F) -> Self {
        Self {
            x: vec.x() as f64,
            y: vec.y() as f64,
        }
    }
}

/// A `Vector2F` is considered to be in screen coordinates as this is the type used in the renderer
impl From<ScreenPoint> for Vector2F {
    fn from(pt: ScreenPoint) -> Self {
        let ScreenPoint {x, y} = pt;
        vec2f(x as f32, y as f32)
    }
}

impl ScreenPoint {
    /// Converts a `Point` in logical (or "world") coordinates to a `ScreenPoint` in
    /// screen coordinates
    ///
    /// # Parameters
    ///
    /// * `point` is the `Point` to convert to screen coordinates
    /// * `dpi_scale` is the high DPI scale factor (>= 0.0)
    /// * `center` is the `Point` configured in the drawing that all other `Point`s are relative to.
    /// * `fb_center` is the center of the framebuffer in screen coordinates.
    pub fn from_logical(
        point: Point,
        dpi_scale: f64,
        center: Point,
        fb_center: ScreenPoint,
    ) -> Self {
        let Point {x, y} = point;
        let Point {x: center_x, y: center_y} = center;
        let ScreenPoint {x: fb_center_x, y: fb_center_y} = fb_center;

        Self {
            x: ((x - center_x) * dpi_scale) as f64 + fb_center_x,
            y: -((y - center_y) * dpi_scale) as f64 + fb_center_y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_coords() {
        // The origin is always at fb_center as long as center is also the origin
        let screen_coord = ScreenPoint::from_logical(Point::origin(), 1.0, Point::origin(), ScreenPoint {x: 200.0, y: 300.0});
        assert_eq!(screen_coord, ScreenPoint {x: 200.0, y: 300.0});
        // The origin is always at fb_center regardless of DPI scale
        let screen_coord = ScreenPoint::from_logical(Point::origin(), 2.0, Point::origin(), ScreenPoint {x: 200.0, y: 300.0});
        assert_eq!(screen_coord, ScreenPoint {x: 200.0, y: 300.0});

        // The x-axis and y-axis treated distinctly and interpreted as cartesian
        let screen_coord = ScreenPoint::from_logical(Point {x: 10.0, y: 20.0}, 1.0, Point::origin(), ScreenPoint {x: 200.0, y: 300.0});
        assert_eq!(screen_coord, ScreenPoint {x: 210.0, y: 280.0});
        // A different fb_center gives a different final coordinate
        let screen_coord = ScreenPoint::from_logical(Point {x: 10.0, y: 20.0}, 1.0, Point::origin(), ScreenPoint {x: 300.0, y: 400.0});
        assert_eq!(screen_coord, ScreenPoint {x: 310.0, y: 380.0});

        // The center is interpreted as cartesian and points are relative to it
        let screen_coord = ScreenPoint::from_logical(Point {x: 10.0, y: 20.0}, 1.0, Point {x: 30.0, y: 5.0}, ScreenPoint {x: 200.0, y: 300.0});
        assert_eq!(screen_coord, ScreenPoint {x: 180.0, y: 285.0});

        // Negative points work too
        let screen_coord = ScreenPoint::from_logical(Point {x: -10.0, y: -20.0}, 1.0, Point {x: 30.0, y: -5.0}, ScreenPoint {x: 200.0, y: 300.0});
        assert_eq!(screen_coord, ScreenPoint {x: 160.0, y: 315.0});

        // DPI scale > 1.0 causes logical coordinates to scale, but NOT screen coordinates
        let screen_coord = ScreenPoint::from_logical(Point {x: 10.0, y: 20.0}, 2.0, Point {x: 30.0, y: 5.0}, ScreenPoint {x: 200.0, y: 300.0});
        assert_eq!(screen_coord, ScreenPoint {x: 160.0, y: 270.0});
    }
}
