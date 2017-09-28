//! Extension traits for various types
use std::time::Duration;

use {Point};

pub trait ToCanvasCoordinates {
    /// Transforms the given local coordinate into a point that can be drawn on the canvas.
    ///
    /// Takes into account the direction of the axis and center when converting
    /// `local` from cartesian coordinates.
    ///
    /// Origin in window is the top left corner and the y-axis goes down instead of up.
    fn to_canvas_coords(self, center: Point) -> Self;
}

impl ToCanvasCoordinates for Point {
    fn to_canvas_coords(self, center: Point) -> Point {
        [center[0] + self[0], center[1] - self[1]]
    }
}

pub trait AsMillis {
    /// Converts the given Duration into its value in milliseconds
    ///
    /// This used to be part of the API but it is easy enough to compute
    /// from the current one as well.
    fn as_millis(&self) -> u64;
}

impl AsMillis for Duration {
    fn as_millis(&self) -> u64 {
        self.as_secs() * 1000 + (self.subsec_nanos() / 1_000_000) as u64
    }
}
