use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use interpolation::Spatial;

/// A point in 2D space
///
/// # Creating a Point
///
/// Methods like [`Turtle::go_to()`], [`Turtle::turn_towards()`] and [`Drawing::set_center()`]
/// all support any type that can be converted into `Point`. That means that you can pass the
/// same value into those methods in several different ways depending on what you prefer:
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Turtle;
/// # fn main() {
/// # let mut turtle = Turtle::new();
/// // These are equivalent
/// turtle.go_to([100.0, 40.0]);
/// turtle.go_to((100.0, 40.0));
/// // This is equivalent too, but the above examples are easier to type
/// use turtle::Point;
/// turtle.go_to(Point {x: 100.0, y: 40.0});
/// # }
/// ```
///
/// Each of these different styles works because the methods call
/// [`.into()`](https://doc.rust-lang.org/std/convert/trait.Into.html) internally.
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Point;
/// # fn main() {
/// assert_eq!(Point {x: 100.0, y: 40.0}, [100.0, 40.0].into());
/// assert_eq!(Point {x: 100.0, y: 40.0}, (100.0, 40.0).into());
/// # }
/// ```
///
/// Notice that we need to convert the right side using `into()` before it can be compared
/// in `assert_eq!()`.
///
/// ```rust,compile_fail,E0308
/// # extern crate turtle;
/// # use turtle::Point;
/// # fn main() {
/// // This will not compile
/// assert_eq!(Point {x: 100.0, y: 40.0}, [100.0, 40.0]);
/// assert_eq!(Point {x: 100.0, y: 40.0}, (100.0, 40.0));
/// # }
/// ```
///
/// # Manipulating Points
///
/// You can add or subtract points and multiply or divide points by scalar (f64) values. There
/// are a variety of [method functions](struct.Point.html#methods) described in the
/// documentation below that provide even more operations.
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Point;
/// # fn main() {
/// # let a = 320.0; let b = 400.0; let c = 70.0; let d = 95.0;
/// // Let's say you have two points with f64 values a, b, c, and d
/// let pt = Point {x: a, y: b};
/// let pt2 = Point {x: c, y: d};
/// assert_eq!(pt + pt2, Point {x: a + c, y: b + d});
/// assert_eq!(pt - pt2, Point {x: a - c, y: b - d});
/// assert_eq!(pt * 2.0, Point {x: a * 2.0, y: b * 2.0});
/// assert_eq!(pt2 / 5.0, Point {x: c / 5.0, y: d / 5.0});
/// assert_eq!(pt2 * 2.0 - pt, Point {x: c * 2.0 - a, y: d * 2.0 - b});
/// # }
/// ```
///
/// # Accessing Point Components
///
/// `Point` supports either using the `x` and `y` fields to access its components or using
/// indexing if you prefer that style.
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Point;
/// # fn main() {
/// let p = Point {x: 100.0, y: 120.0};
///
/// // Get x coordinate
/// let x = p.x;
/// assert_eq!(x, 100.0);
/// assert_eq!(p[0], x);
///
/// // Get y coordinate
/// let y = p.y;
/// assert_eq!(y, 120.0);
/// assert_eq!(p[1], y);
///
/// // With pattern matching
/// let Point {x, y} = p;
/// assert_eq!(x, 100.0);
/// assert_eq!(y, 120.0);
///
/// // Modifying x and y
/// let mut pt: Point = [240.0, 430.0].into();
/// # assert_eq!(pt.x, 240.0);
/// # assert_eq!(pt.y, 430.0);
/// pt.x = 73.0;
/// pt.y = 89.0;
/// assert_eq!(pt.x, 73.0);
/// assert_eq!(pt.y, 89.0);
/// // Using indexing
/// let mut pt2: Point = [100.0, 200.0].into();
/// # assert_eq!(pt2.x, 100.0);
/// # assert_eq!(pt2.y, 200.0);
/// pt2[0] = pt.x;
/// pt2[1] = pt.y;
/// assert_eq!(pt2.x, 73.0);
/// assert_eq!(pt2.y, 89.0);
/// # }
/// ```
///
/// [`Turtle::go_to()`]: struct.Turtle.html#method.go_to
/// [`Turtle::turn_towards()`]: struct.Turtle.html#method.turn_towards
/// [`Drawing::set_center()`]: struct.Drawing.html#method.set_center
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    /// The x-coordinate of the Point
    pub x: f64,
    /// The y-coordinate of the Point
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
    /// The length of a point is defined as `sqrt(x^2 + y^2)`
    pub fn square_len(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    /// Returns the length of this point.
    ///
    /// The length of a point is defined as `sqrt(x^2 + y^2)`
    pub fn len(self) -> f64 {
        self.square_len().sqrt()
    }

    /// Computes the four quadrant arctangent of `self.y` and `self.x`.
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

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Invalid coordinate for Point: {}", index),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Invalid coordinate for Point: {}", index),
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
