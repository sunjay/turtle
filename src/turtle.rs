use radians::{self, Radians};
use turtle_window::TurtleWindow;
use event::MouseButton;
use {Speed, Color, Event};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AngleUnit {
    Degrees,
    Radians,
}

impl AngleUnit {
    fn to_radians(&self, angle: Angle) -> Radians {
        match *self {
            AngleUnit::Degrees => Radians::from_degrees_value(angle),
            AngleUnit::Radians => Radians::from_radians_value(angle),
        }
    }

    fn to_angle(&self, angle: Radians) -> Angle {
        match *self {
            AngleUnit::Degrees => angle.to_degrees(),
            AngleUnit::Radians => angle.to_radians(),
        }
    }
}

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

/// Any distance value
pub type Distance = f64;

/// An angle value without a unit
///
/// The unit of the angle represented by this value depends on what
/// unit the Turtle was set to when this angle was retrieved
pub type Angle = f64;

/// A turtle with a pen attached to its tail.
///
/// **The idea:** You control a turtle with a pen tied to its tail. As it moves
/// across the screen, it draws the path that it follows. You can use this to draw
/// any picture you want just by moving the turtle across the screen.
///
/// ![turtle moving forward](https://github.com/sunjay/turtle/raw/master/forward.gif)
///
/// See the documentation for the methods below to learn about the different drawing commands you
/// can use with the turtle.
pub struct Turtle {
    window: TurtleWindow,
    angle_unit: AngleUnit,
}

impl Turtle {
    /// Create a new turtle.
    ///
    /// This will immediately open a new window with the turtle at the center. As each line in
    /// your program runs, the turtle shown in the window will update.
    ///
    /// ```rust
    /// # #![allow(unused_variables, unused_mut)]
    /// extern crate turtle;
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     // Do things with the turtle...
    /// }
    /// ```
    pub fn new() -> Turtle {
        Turtle {
            window: TurtleWindow::new(),
            angle_unit: AngleUnit::Degrees,
        }
    }

    /// Returns the current speed of the turtle
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.set_speed(8);
    /// assert_eq!(turtle.speed(), Speed::Eight);
    /// # }
    /// ```
    pub fn speed(&self) -> Speed {
        self.window.turtle().speed
    }

    /// Set the turtle's movement speed to the given setting. This speed affects the animation of
    /// the turtle's movement and rotation. The turtle's speed is limited to values between 0 and
    /// 10. If you pass in values that are not integers or outside of that range, the closest
    /// possible value will be chosen.
    ///
    /// This method's types make it so that it can be called in a number of different ways:
    ///
    /// ```rust,no_run
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.set_speed("normal");
    /// turtle.set_speed("fast");
    /// turtle.set_speed(2);
    /// turtle.set_speed(10);
    /// // Directly using a Speed variant works, but the methods above are usually more convenient.
    /// turtle.set_speed(Speed::Six);
    /// # }
    /// ```
    ///
    /// If input is a number greater than 10 or smaller than 1,
    /// speed is set to 0 (`Speed::Instant`). Strings are converted as follows:
    ///
    /// | String      | Value          |
    /// | ----------- | -------------- |
    /// | `"slowest"` | `Speed::One`     |
    /// | `"slow"`    | `Speed::Three`   |
    /// | `"normal"`  | `Speed::Six`     |
    /// | `"fast"`    | `Speed::Eight`   |
    /// | `"fastest"` | `Speed::Ten`     |
    /// | `"instant"` | `Speed::Instant` |
    ///
    /// Anything else will cause the program to `panic!` at runtime.
    ///
    /// ## Moving Instantly
    ///
    /// A speed of zero (`Speed::Instant`) results in no animation. The turtle moves instantly
    /// and turns instantly. This is very useful for moving the turtle from its "home" position
    /// before you start drawing. By setting the speed to instant, you don't have to wait for
    /// the turtle to move into position.
    ///
    /// ## Learning About Conversion Traits
    ///
    /// Using this method is an excellent way to learn about conversion
    /// traits `From` and `Into`. This method takes a *generic type* as its speed parameter. That type
    /// is specified to implement the `Into` trait for the type `Speed`. That means that *any* type
    /// that can be converted into a `Speed` can be passed to this method.
    ///
    /// We have implemented that trait for several types like strings and 32-bit integers so that
    /// those values can be passed into this method.
    /// Rather than calling this function and passing `Speed::Six` directly, you can use just `6`.
    /// Rust will then allow us to call `.into()` as provided by the `Into<Speed>` trait to get the
    /// corresponding `Speed` value.
    ///
    /// You can pass in strings, 32-bit integers, and even `Speed` enum variants because they all
    /// implement the `Into<Speed>` trait.
    pub fn set_speed<S: Into<Speed>>(&mut self, speed: S) {
        self.window.turtle_mut().speed = speed.into();
    }

    /// Returns the turtle's current location (x, y)
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.forward(100.0);
    /// let pos = turtle.position();
    /// # // Cheating a bit here for rounding...
    /// # let pos = [pos[0].round(), pos[1].round()];
    /// assert_eq!(pos, [0.0, 100.0]);
    /// # }
    /// ```
    pub fn position(&self) -> Point {
        self.window.turtle().position
    }

    /// Returns the turtle's current heading.
    ///
    /// Units are by default degrees, but can be set using the
    /// [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
    /// [`use_radians()`](struct.Turtle.html#method.use_radians) methods.
    ///
    /// The heading is relative to the positive x axis (east). When first created, the turtle
    /// starts facing north. That means that its heading is 90.0 degrees. The following chart
    /// contains many common directions and their angles.
    ///
    /// | Cardinal Direction | Heading (degrees) | Heading (radians) |
    /// | ------------------ | ----------------- | ----------------- |
    /// | East               | 0.0&deg;          | `0.0`             |
    /// | North              | 90.0&deg;         | `PI/2`            |
    /// | West               | 180.0&deg;        | `PI`              |
    /// | South              | 270.0&deg;        | `3*PI/2`          |
    ///
    /// You can test the result of `heading()` with these values to see if the turtle is facing
    /// a certain direction.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// // Turtles start facing north
    /// let mut turtle = Turtle::new();
    /// // The rounding is to account for floating-point error
    /// assert_eq!(turtle.heading().round(), 90.0);
    /// turtle.right(31.0);
    /// assert_eq!(turtle.heading().round(), 59.0);
    /// turtle.left(193.0);
    /// assert_eq!(turtle.heading().round(), 252.0);
    /// turtle.left(130.0);
    /// // Angles should not exceed 360.0
    /// assert_eq!(turtle.heading().round(), 22.0);
    /// # }
    /// ```
    pub fn heading(&self) -> Angle {
        let heading = self.window.turtle().heading;
        self.angle_unit.to_angle(heading)
    }

    /// Returns true if `Angle` values will be interpreted as degrees.
    ///
    /// See [`use_degrees()`](struct.Turtle.html#method.use_degrees) for more information.
    pub fn is_using_degrees(&self) -> bool {
        self.angle_unit == AngleUnit::Degrees
    }

    /// Returns true if `Angle` values will be interpreted as radians.
    ///
    /// See [`use_radians()`](struct.Turtle.html#method.use_degrees) for more information.
    pub fn is_using_radians(&self) -> bool {
        self.angle_unit == AngleUnit::Radians
    }

    /// Change the angle unit to degrees.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// # turtle.use_radians();
    /// assert!(!turtle.is_using_degrees());
    /// turtle.use_degrees();
    /// assert!(turtle.is_using_degrees());
    /// # }
    /// ```
    pub fn use_degrees(&mut self) {
        self.angle_unit = AngleUnit::Degrees;
    }

    /// Change the angle unit to radians.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// assert!(!turtle.is_using_radians());
    /// turtle.use_radians();
    /// assert!(turtle.is_using_radians());
    /// # }
    /// ```
    pub fn use_radians(&mut self) {
        self.angle_unit = AngleUnit::Radians;
    }

    /// Return true if pen is down, false if it’s up.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// assert!(turtle.is_pen_down());
    /// turtle.pen_up();
    /// assert!(!turtle.is_pen_down());
    /// turtle.pen_down();
    /// assert!(turtle.is_pen_down());
    /// # }
    /// ```
    pub fn is_pen_down(&self) -> bool {
        self.window.drawing().pen.enabled
    }

    /// Pull the pen down so that the turtle draws while moving.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// # turtle.pen_up();
    /// assert!(!turtle.is_pen_down());
    /// // This will move the turtle, but not draw any lines
    /// turtle.forward(100.0);
    /// turtle.pen_down();
    /// assert!(turtle.is_pen_down());
    /// // The turtle will now draw lines again
    /// turtle.forward(100.0);
    /// # }
    /// ```
    pub fn pen_down(&mut self) {
        self.window.drawing_mut().pen.enabled = true;
    }

    /// Pick the pen up so that the turtle does not draw while moving
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// assert!(turtle.is_pen_down());
    /// // The turtle will move and draw a line
    /// turtle.forward(100.0);
    /// turtle.pen_up();
    /// assert!(!turtle.is_pen_down());
    /// // Now, the turtle will move, but not draw anything
    /// turtle.forward(100.0);
    /// # }
    /// ```
    pub fn pen_up(&mut self) {
        self.window.drawing_mut().pen.enabled = false;
    }

    /// Returns the size (thickness) of the pen. The thickness is measured in pixels.
    ///
    /// See [`set_pen_size()`](struct.Turtle.html#method.set_pen_size) for more details.
    pub fn pen_size(&self) -> f64 {
        self.window.drawing().pen.thickness
    }

    /// Sets the thickness of the pen to the given size. The thickness is measured in pixels.
    ///
    /// The turtle's pen has a flat tip. The value you set the pen's size to will change the
    /// width of the stroke created by the turtle as it moves. See the example below for more
    /// about what this means.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///
    ///     turtle.pen_up();
    ///     turtle.right(90.0);
    ///     turtle.backward(300.0);
    ///     turtle.pen_down();
    ///
    ///     turtle.set_pen_color("#2196F3"); // blue
    ///     turtle.set_pen_size(1.0);
    ///     turtle.forward(200.0);
    ///
    ///     turtle.set_pen_color("#f44336"); // red
    ///     turtle.set_pen_size(50.0);
    ///     turtle.forward(200.0);
    ///
    ///     turtle.set_pen_color("#4CAF50"); // green
    ///     turtle.set_pen_size(100.0);
    ///     turtle.forward(200.0);
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle pen thickness](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/pen_thickness.png)
    ///
    /// Notice that while the turtle travels in a straight line, it produces different thicknesses
    /// of lines which appear like large rectangles.
    pub fn set_pen_size(&mut self, thickness: f64) {
        self.window.drawing_mut().pen.thickness = thickness;
    }

    /// Returns the color of the pen.
    ///
    /// See the [`color` module](color/index.html) for more information about colors.
    pub fn pen_color(&self) -> Color {
        self.window.drawing().pen.color
    }

    /// Sets the color of the pen to the given color.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`color` module](color/index.html) for more information.
    pub fn set_pen_color<C: Into<Color>>(&mut self, color: C) {
        self.window.drawing_mut().pen.color = color.into();
    }

    /// Returns the color of the background.
    ///
    /// See the [`color` module](color/index.html) for more information about colors.
    pub fn background_color(&self) -> Color {
        self.window.drawing().background
    }

    /// Sets the color of the background to the given color.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`color` module](color/index.html) for more information.
    pub fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        self.window.drawing_mut().background = color.into();
    }

    /// Returns the current fill color.
    ///
    /// This will be used to fill the shape when
    /// [`begin_fill()`](struct.Turtle.html#method.begin_fill) and
    /// [`end_fill()`](struct.Turtle.html#method.end_fill) are called.
    ///
    /// See the [`color` module](color/index.html) for more information about colors.
    pub fn fill_color(&self) -> Color {
        self.window.drawing().fill_color
    }

    /// Sets the fill color to the given color.
    ///
    /// **Note:** The fill color set **before** `begin_fill()` is called will be used to fill
    /// the shape.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`color` module](color/index.html) for more information.
    pub fn set_fill_color<C: Into<Color>>(&mut self, color: C) {
        self.window.drawing_mut().fill_color = color.into();
    }

    /// Begin filling the shape drawn by the turtle's movements.
    ///
    /// **Rule of thumb:** For every call to [`begin_fill()`](struct.Turtle.html#method.begin_fill),
    /// there should be a corresponding call to [`end_fill()`](struct.Turtle.html#method.end_fill).
    ///
    /// # Example
    ///
    /// The following example will draw a circle filled with the color red and then a square with
    /// no fill.
    ///
    /// **Note:** The fill color set **before** `begin_fill()` is called will be used to fill
    /// the shape.
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    ///
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.right(90.0);
    ///     turtle.set_pen_size(3.0);
    ///
    ///     turtle.set_pen_color("blue");
    ///     turtle.set_fill_color("red");
    ///     turtle.begin_fill();
    ///     for _ in 0..360 {
    ///         turtle.forward(2.0);
    ///         turtle.right(1.0);
    ///     }
    ///     turtle.end_fill();
    ///
    ///     turtle.set_pen_color("green");
    ///     turtle.forward(120.0);
    ///     for _ in 0..3 {
    ///         turtle.right(90.0);
    ///         turtle.forward(240.0);
    ///     }
    ///     turtle.right(90.0);
    ///     turtle.forward(120.0);
    /// }
    /// ```
    ///
    /// This will result in the following:
    ///
    /// ![turtle fill example](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/red_circle.png)
    pub fn begin_fill(&mut self) {
        self.window.begin_fill();
    }

    /// Stop filling the shape drawn by the turtle's movements.
    ///
    /// **Rule of thumb:** For every call to [`begin_fill()`](struct.Turtle.html#method.begin_fill),
    /// there should be a corresponding call to [`end_fill()`](struct.Turtle.html#method.end_fill).
    ///
    /// See [`begin_fill()`](struct.Turtle.html#method.begin_fill) for more information.
    pub fn end_fill(&mut self) {
        self.window.end_fill();
    }

    /// Returns true if the turtle is visible.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// let mut turtle = Turtle::new();
    /// assert!(turtle.is_visible());
    /// turtle.hide();
    /// assert!(!turtle.is_visible());
    /// turtle.show();
    /// assert!(turtle.is_visible());
    /// # }
    /// ```
    pub fn is_visible(&self) -> bool {
        self.window.turtle().visible
    }

    /// Makes the turtle invisible. The shell will not be shown, but drawings will continue.
    ///
    /// Useful for some complex drawings.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// assert!(turtle.is_visible());
    /// turtle.hide();
    /// assert!(!turtle.is_visible());
    /// # }
    /// ```
    pub fn hide(&mut self) {
        self.window.turtle_mut().visible = false;
    }

    /// Makes the turtle visible.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// # turtle.hide();
    /// assert!(!turtle.is_visible());
    /// turtle.show();
    /// assert!(turtle.is_visible());
    /// # }
    /// ```
    pub fn show(&mut self) {
        self.window.turtle_mut().visible = true;
    }

    /// Delete the turtle's drawings from the screen.
    ///
    /// Does not move turtle. Position, speed and heading of the turtle are not affected. The
    /// background color and any other settings (pen color, size, etc.) all remain the same.
    pub fn clear(&mut self) {
        self.window.clear();
    }

    /// Move the turtle forward by the given amount of `distance`. If the pen is down, the turtle
    /// will draw a line as it moves.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move backward
    /// using this method.
    pub fn forward(&mut self, distance: Distance) {
        self.window.forward(distance);
    }

    /// Move the turtle backward by the given amount of `distance`. If the pen is down, the turtle
    /// will draw a line as it moves.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move forwards
    /// using this method.
    pub fn backward(&mut self, distance: Distance) {
        // Moving backwards is essentially moving forwards with a negative distance
        self.window.forward(-distance);
    }

    /// Rotate the turtle right (clockwise) by the given angle. Since the turtle rotates in place,
    /// its position will not change and it will not draw anything at all.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
    /// [`use_radians()`](struct.Turtle.html#method.use_radians).
    pub fn right(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, true);
    }

    /// Rotate the turtle left (counterclockwise) by the given angle. Since the turtle rotates
    /// in place, its position will not change and it will not draw anything at all.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
    /// [`use_radians()`](struct.Turtle.html#method.use_radians).
    pub fn left(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, false);
    }

    /// Rotates the turtle to face the given coordinates.
    /// Coordinates are relative to the center of the window.
    ///
    /// If the coordinates are the same as the turtle's current position, no rotation takes place.
    /// Always rotates the least amount necessary in order to face the given point.
    ///
    /// ## UNSTABLE
    /// This feature is currently unstable and completely buggy. Do not use it until it is fixed.
    pub fn turn_towards(&mut self, target: Point) {
        let target_x = target[0];
        let target_y = target[1];

        let position = self.position();
        let x = position[0];
        let y = position[1];

        if (target_x - x).abs() < 0.1 && (target_y - y).abs() < 0.1 {
            return;
        }

        let heading = self.window.turtle().heading;

        let angle = (target_y - y).atan2(target_x - x);
        let angle = Radians::from_radians_value(angle);
        let angle = (angle - heading) % radians::TWO_PI;
        // Try to rotate as little as possible
        let angle = if angle.abs() > radians::PI {
            // Using signum to deal with negative angles properly
            angle.signum()*(radians::TWO_PI - angle.abs())
        }
        else {
            angle
        };
        self.window.rotate(angle, false);
    }

    /// Returns the next event (if any).
    //TODO: Example of usage with an event loop
    pub fn poll_event(&mut self) -> Option<Event> {
        self.window.poll_event()
    }

    /// Convenience function that waits for a click to occur before returning.
    ///
    /// Useful for when you want your program to wait for the user to click before continuing so
    /// that it doesn't start right away.
    pub fn wait_for_click(&mut self) {
        loop {
            if let Some(Event::MouseButtonReleased(MouseButton::Left)) = self.poll_event() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_using_radians_degrees() {
        // is_using_radians and is_using_degrees should be inverses of each other
        let mut turtle = Turtle::new();
        assert!(!turtle.is_using_radians());
        assert!(turtle.is_using_degrees());
        turtle.use_radians();
        assert!(turtle.is_using_radians());
        assert!(!turtle.is_using_degrees());
        turtle.use_degrees();
        assert!(!turtle.is_using_radians());
        assert!(turtle.is_using_degrees());
    }
}
