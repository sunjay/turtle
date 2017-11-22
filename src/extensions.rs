//! Extension traits for various types
use std::time::Duration;

use types::Point;

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
        [center[0] + self[0], center[1] - self[1]]
    }

    fn to_local_coords(self, center: Self) -> Self {
        [self[0] - center[0], center[1] - self[1]]
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
        self.as_secs() * 1000 + u64::from(self.subsec_nanos() / 1_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_screen_coords() {
        let center: Point = [300.0, 400.0];

        let point: Point = [20.0, 30.0];
        assert_eq!(point.to_screen_coords(center), [320.0, 370.0]);
        assert_eq!(point.to_local_coords(center), [-280.0, 370.0]);
    }
}
