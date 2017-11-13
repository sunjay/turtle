use std::f64::consts::PI as fPI;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

use interpolation::Spatial;

pub const PI: Radians = Radians(fPI);
pub const TWO_PI: Radians = Radians(2.*fPI);
pub const ZERO: Radians = Radians(0.0);

/// Represents an angle in radians.
///
/// No way to use the Radians(value) constructor because we want to force
/// you to think about whether your value is actually in radians or degrees.
/// Use `Radians::from_degrees_value` or `Radians::from_radians_value` to
/// construct an instance.
#[derive(Default, Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct Radians(f64);

impl Radians {
    /// Create a new instance of Radians from a value in radians
    pub fn from_radians_value(radians: f64) -> Radians {
        Radians(radians)
    }

    /// Convert a value from degrees to radians and then create an instance
    /// of Radians
    pub fn from_degrees_value(degrees: f64) -> Radians {
        Radians(degrees.to_radians())
    }

    /// Returns the raw value stored in radians
    pub fn to_radians(&self) -> f64 {
        self.0
    }

    /// Returns the raw value stored in degrees
    pub fn to_degrees(&self) -> f64 {
        self.0.to_degrees()
    }

    /// Computes the cosine of this radians value
    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    /// Computes the sine of this radians value
    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    /// Returns true if this value is positive infinity or negative infinity and false otherwise.
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Returns true if this value is `NaN` and false otherwise.
    pub fn is_nan(self) -> bool {
        self.0.is_infinite()
    }

    /// See [`std::f64::signum()`](https://doc.rust-lang.org/std/primitive.f64.html#method.signum)
    pub fn signum(self) -> f64 {
        self.0.signum()
    }

    /// See [`std::f64::abs()`](https://doc.rust-lang.org/std/primitive.f64.html#method.abs)
    pub fn abs(self) -> Self {
        Radians(self.0.abs())
    }

    /// See [`std::f64::floor()`](https://doc.rust-lang.org/std/primitive.f64.html#method.floor)
    pub fn floor(self) -> Self {
        Radians(self.0.floor())
    }
}

impl Spatial for Radians {
    type Scalar = f64;

    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }

    fn scale(&self, other: &Self::Scalar) -> Self {
        *self * *other
    }
}

impl Add for Radians {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Radians(self.0 + other.0)
    }
}

impl Sub for Radians {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Radians(self.0 - other.0)
    }
}

impl Mul for Radians {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Radians(self.0 * other.0)
    }
}

impl Mul<f64> for Radians {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Radians(self.0 * other)
    }
}

impl Mul<Radians> for f64 {
    type Output = Radians;

    fn mul(self, other: Radians) -> Radians {
        Radians(self * other.0)
    }
}

impl Div for Radians {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Radians(self.0 / other.0)
    }
}

impl Div<f64> for Radians {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Radians(self.0 / other)
    }
}

impl Rem for Radians {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Radians(self.0 % other.0)
    }
}

impl Neg for Radians {
    type Output = Self;

    fn neg(self) -> Self {
        Radians(-self.0)
    }
}
