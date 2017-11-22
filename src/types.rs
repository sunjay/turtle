/// A point in 2D space: [x, y]
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Point;
/// # fn main() {
/// let p: Point = [100., 120.];
/// // get x coordinate
/// let x = p[0];
/// assert_eq!(x, 100.);
/// // get y coordinate
/// let y = p[1];
/// assert_eq!(y, 120.);
/// # }
pub type Point = [f64; 2];

/// Any distance value (positive or negative)
pub type Distance = f64;

/// An angle value without a unit
///
/// The unit with which this angle will be interpreted depends on whether the Turtle is set to use
/// degrees or radians. See the [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
/// [`use_radians()`](struct.Turtle.html#method.use_radians) methods for more information.
pub type Angle = f64;
