use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use interpolation::Spatial;

/// A point in 2D space
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Point;
/// # fn main() {
/// let p = Point {x: 100., y: 120.};
/// // get x coordinate
/// let x = p.x;
/// assert_eq!(x, 100.);
/// // get y coordinate
/// let y = p.y;
/// assert_eq!(y, 120.);
/// # }
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Returns a Point that represents the origin of the coordinate system.
    ///
    /// For our "cartesian" coordinate system, this is always (0.0, 0.0)
    pub fn origin() -> Self {
        Point {x: 0.0, y: 0.0}
    }

    /// Returns true if both x and y are finite (neither infinite nor `NaN`).
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Returns true if both x and y are neither zero, infinite,
    /// [subnormal](https://en.wikipedia.org/wiki/Denormal_number), or `NaN`.
    pub fn is_normal(self) -> bool {
        self.x.is_normal() && self.y.is_normal()
    }

    /// Returns true if both x and y are either zero, infinite,
    /// [subnormal](https://en.wikipedia.org/wiki/Denormal_number), or `NaN`.
    pub fn is_not_normal(self) -> bool {
        !self.x.is_normal() && !self.y.is_normal()
    }

    /// Computes the absolute value of x and y and returns a new `Point`
    pub fn abs(self) -> Self {
        Point {x: self.x.abs(), y: self.y.abs()}
    }

    /// Returns a new `Point` with x and y set to the nearest integer to each of their values.
    /// Rounds half-way cases away from 0.0.
    pub fn round(self) -> Self {
        Point {x: self.x.round(), y: self.y.round()}
    }

    /// Returns the square of the length of this point.
    ///
    /// The length of a point is defined as sqrt(x^2 + y^2)
    pub fn square_len(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    /// Returns the length of this point.
    ///
    /// The length of a point is defined as sqrt(x^2 + y^2)
    pub fn len(self) -> f64 {
        self.square_len().sqrt()
    }

    /// Computes the four quadrant arctangent of self.y and self.x.
    pub fn atan2(self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl From<(f64, f64)> for Point {
    fn from(pt: (f64, f64)) -> Self {
        Point {x: pt.0, y: pt.1}
    }
}

impl From<[f64; 2]> for Point {
    fn from(pt: [f64; 2]) -> Self {
        Point {x: pt[0], y: pt[1]}
    }
}

impl From<Point> for [f64; 2] {
    fn from(pt: Point) -> Self {
        [pt.x, pt.y]
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Self::Output {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Self::Output {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Spatial for Point {
    type Scalar = f64;

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    #[inline(always)]
    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }

    #[inline(always)]
    fn scale(&self, scalar: &Self::Scalar) -> Self {
        *self * *scalar
    }
}
