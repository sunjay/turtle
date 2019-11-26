use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use serde::{Serialize, Deserialize};
use interpolation::Lerp;

use crate::rand::{Random, RandomRange};

/// A point in 2D space
///
/// # Creating a Point
///
/// Methods like [`Turtle::go_to()`], [`Turtle::turn_towards()`] and [`Drawing::set_center()`]
/// all support any type that can be converted into `Point`. That means that you can pass the
/// same value into those methods in several different ways depending on what you prefer:
///
/// ```rust
/// # use turtle::Turtle;
/// # let mut turtle = Turtle::new();
/// // These are equivalent
/// turtle.go_to([100.0, 40.0]);
/// turtle.go_to((100.0, 40.0));
/// // This is equivalent too, but the above examples are easier to type
/// use turtle::Point;
/// turtle.go_to(Point {x: 100.0, y: 40.0});
/// ```
///
/// Each of these different styles works because the methods call
/// [`.into()`](https://doc.rust-lang.org/std/convert/trait.Into.html) internally.
///
/// ```rust
/// # use turtle::Point;
/// assert_eq!(Point {x: 100.0, y: 40.0}, [100.0, 40.0].into());
/// assert_eq!(Point {x: 100.0, y: 40.0}, (100.0, 40.0).into());
/// ```
///
/// Notice that we need to convert the right side using `into()` before it can be compared
/// in `assert_eq!()`.
///
/// ```rust,compile_fail,E0308
/// # use turtle::Point;
/// // This will not compile
/// assert_eq!(Point {x: 100.0, y: 40.0}, [100.0, 40.0]);
/// assert_eq!(Point {x: 100.0, y: 40.0}, (100.0, 40.0));
/// ```
///
/// # Manipulating Points
///
/// You can add or subtract points and multiply or divide points by scalar (f64) values. There
/// are a variety of [method functions](struct.Point.html#methods) described in the
/// documentation below that provide even more operations.
///
/// ```rust
/// # use turtle::Point;
/// # let a = 320.0; let b = 400.0; let c = 70.0; let d = 95.0;
/// // Let's say you have two points with f64 values a, b, c, and d
/// let pt = Point {x: a, y: b};
/// let pt2 = Point {x: c, y: d};
/// assert_eq!(pt + pt2, Point {x: a + c, y: b + d});
/// assert_eq!(pt - pt2, Point {x: a - c, y: b - d});
/// assert_eq!(pt * 2.0, Point {x: a * 2.0, y: b * 2.0});
/// assert_eq!(pt2 / 5.0, Point {x: c / 5.0, y: d / 5.0});
/// assert_eq!(pt2 * 2.0 - pt, Point {x: c * 2.0 - a, y: d * 2.0 - b});
/// ```
///
/// # Accessing Point Components
///
/// `Point` supports either using the `x` and `y` fields to access its components or using
/// indexing if you prefer that style.
///
/// ```rust
/// # use turtle::Point;
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
/// ```
///
/// # Generating Random Points
///
/// Use the [`random()`] function to generate random points. The values of `x` and `y` will be
/// between `0.0` and `1.0` (inclusive).
///
/// ```rust
/// use turtle::{Point, rand::random};
///
/// let pt: Point = random();
/// assert!(pt.x >= 0.0 && pt.x <= 1.0);
/// assert!(pt.y >= 0.0 && pt.y <= 1.0);
/// ```
///
/// When [`random_range()`] is used to generate a `Point`, it creates a random point within the
/// rectangle formed by the two points given as arguments to [`random_range()`].
///
/// ```rust
/// use turtle::{Point, rand::random_range};
///
/// // Generates a Point value with:
/// //   x-coordinate between 46.0 and 92.0
/// //   y-coordinate between 39.0 and 103.0
/// let value: Point = random_range::<_, Point>([92.0, 39.0].into(), [46.0, 103.0].into());
/// assert!(value.x >= 46.0 && value.x <= 92.0);
/// assert!(value.y >= 39.0 && value.y <= 103.0);
/// ```
///
/// [`Turtle::go_to()`]: struct.Turtle.html#method.go_to
/// [`Turtle::turn_towards()`]: struct.Turtle.html#method.turn_towards
/// [`Drawing::set_center()`]: struct.Drawing.html#method.set_center
/// [`random()`]: rand/fn.random.html
/// [`random_range()`]: rand/fn.random_range.html
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
        Self { x: 0.0, y: 0.0 }
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
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a new `Point` with x and y set to the nearest integer to each of their values.
    /// Rounds half-way cases away from 0.0.
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Returns the minimum x and y coordinates of the two points
    ///
    /// ```rust
    /// # use turtle::Point;
    /// let p1 = Point {x: 100.0, y: 203.18};
    /// let p2 = Point {x: 3.0, y: 1029.677};
    /// assert_eq!(p1.min(p2), Point {x: 3.0, y: 203.18});
    /// ```
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Returns the maximum x and y coordinates of the two points
    ///
    /// ```rust
    /// # use turtle::Point;
    /// let p1 = Point {x: 100.0, y: 203.18};
    /// let p2 = Point {x: 3.0, y: 1029.677};
    /// assert_eq!(p1.max(p2), Point {x: 100.0, y: 1029.677});
    /// ```
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
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
        Self { x: pt.0, y: pt.1 }
    }
}

impl From<[f64; 2]> for Point {
    fn from(pt: [f64; 2]) -> Self {
        Self { x: pt[0], y: pt[1] }
    }
}

impl From<Point> for [f64; 2] {
    fn from(pt: Point) -> Self {
        [pt.x, pt.y]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Self::Output {
        other * self
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
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

impl Lerp for Point {
    type Scalar = f64;

    #[inline(always)]
    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        Self {
            x: self.x.lerp(&other.x, &scalar),
            y: self.y.lerp(&other.y, &scalar),
        }
    }
}

impl Random for Point {
    fn random() -> Self {
        Point {
            x: Random::random(),
            y: Random::random(),
        }
    }
}

impl<B: Into<Point>> RandomRange<B> for Point {
    fn random_range(p1: B, p2: B) -> Self {
        let p1 = p1.into();
        let p2 = p2.into();

        let min = p1.min(p2);
        let max = p1.max(p2);

        Point {
            x: RandomRange::random_range(min.x, max.x),
            y: RandomRange::random_range(min.y, max.y),
        }
    }
}
