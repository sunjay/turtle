use std::fmt::{self, Debug};

use crate::{Color, Point, Speed, Distance, Angle};
use crate::async_turtle::AsyncTurtle;
use crate::sync_runtime::block_on;

/// A turtle with a pen attached to its tail
///
/// **The idea:** You control a turtle with a pen tied to its tail. As it moves
/// across the screen, it draws the path that it follows. You can use this to draw
/// any picture you want just by moving the turtle across the screen.
///
/// ![turtle moving forward](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/forward.gif)
///
/// See the documentation for the methods below to learn about the different drawing commands you
/// can use with the turtle.
pub struct Turtle {
    turtle: AsyncTurtle,
}

impl Debug for Turtle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = block_on(self.turtle.debug());
        Debug::fmt(&state, f)
    }
}

impl Default for Turtle {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AsyncTurtle> for Turtle {
    fn from(turtle: AsyncTurtle) -> Self {
        Self {turtle}
    }
}

impl Turtle {
    /// Create a new turtle.
    ///
    /// This will immediately open a new window with the turtle at the center. As each line in
    /// your program runs, the turtle shown in the window will update.
    ///
    /// ```rust,no_run
    /// # #![allow(unused_variables, unused_mut)]
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     // Do things with the turtle...
    /// }
    /// ```
    ///
    /// **Note:** If you do not create the `Turtle` right at the beginning of `main()`, call
    /// [`turtle::start()`](fn.start.html) in order to avoid any problems.
    pub fn new() -> Turtle {
        // This needs to be called as close to the start of the program as possible. We call it
        // here since Turtle::new() or AsyncTurtle::new() are commonly called at the beginning
        // of many programs that use the turtle crate.
        // We can't call start() within another tokio runtime, so we have to do it before block_on
        crate::start();

        Turtle {
            turtle: block_on(AsyncTurtle::new()),
        }
    }

    /// Move the turtle forward by the given amount of `distance`. If the pen is down, the turtle
    /// will draw a line as it moves.
    ///
    /// The turtle takes very small steps (measured in "pixels"). So if you want it to move more,
    /// use a bigger value to make the turtle walk further.
    /// The `distance` can be a negative value in which case the turtle will move backward.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use turtle::Turtle;
    /// # let mut turtle = Turtle::new();
    /// // Move forward 10 tiny turtle steps, drawing a line as you move
    /// turtle.forward(10.0);
    ///
    /// // Move forward 100 tiny turtle steps, drawing a much longer line
    /// turtle.forward(100.0);
    ///
    /// // Move backward 223 tiny turtle steps, without drawing anything
    /// turtle.pen_up();
    /// turtle.forward(-223.0);
    /// # assert_eq!(turtle.position().y.round(), -113.0);
    /// ```
    pub fn forward(&mut self, distance: Distance) {
        block_on(self.turtle.forward(distance))
    }

    /// Move the turtle backwards by the given amount of `distance`. If the pen is down, the turtle
    /// will draw a line as it moves.
    ///
    /// The turtle takes very small steps (measured in "pixels"). So if you want it to move more,
    /// use a bigger value to make the turtle walk further.
    /// The `distance` can be a negative value in which case the turtle will move forward.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use turtle::Turtle;
    /// # let mut turtle = Turtle::new();
    /// // Move backward 10 tiny turtle steps, drawing a line as you move
    /// turtle.backward(10.0);
    ///
    /// // Move backward 100 tiny turtle steps, drawing a much longer line
    /// turtle.backward(100.0);
    ///
    /// // Move forward 179 tiny turtle steps, without drawing anything
    /// turtle.pen_up();
    /// turtle.backward(-179.0);
    /// # assert_eq!(turtle.position().y.round(), 69.0);
    /// ```
    pub fn backward(&mut self, distance: Distance) {
        block_on(self.turtle.backward(distance))
    }

    /// Instruct the turtle to turn right (clockwise) by the given angle. Since the turtle rotates
    /// in place, its position will not change and it will not draw anything while it turns.
    ///
    /// The `angle` parameter is a floating point number that represents how much you want the
    /// turtle to rotate.
    /// The unit of `angle` is "degrees" by default. You can change that by using the
    /// [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
    /// [`use_radians()`](struct.Turtle.html#method.use_radians) methods.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// // rotate right by 30 degrees
    /// turtle.right(30.0);
    ///
    /// // rotate right by 1 radian (57.2957795 degrees)
    /// turtle.use_radians();
    /// turtle.right(1.0);
    /// // Use PI for precise angles in radians
    /// use std::f64::consts::PI;
    /// // This is the same as turning 45.0 degrees
    /// turtle.right(PI/4.0);
    /// # // Calculate the angle that should result from the above rotations
    /// # let expected = (90f64 - 30f64).to_radians() - 1.0 - PI/4.0;
    /// # // Need to properly normalize the angle so that it can be checked
    /// # // We only perform a normalization in `right`, and not `left` because the angle resulting
    /// # // from the rotations is negative.
    /// # let expected = expected - (2.0*PI) * (expected / (2.0*PI)).floor();
    /// # let expected = (expected * 1e5).trunc();
    /// # assert_eq!((turtle.heading() * 1e5).trunc(), expected);
    /// ```
    pub fn right(&mut self, angle: Angle) {
        block_on(self.turtle.right(angle))
    }

    /// Instruct the turtle to turn left (counterclockwise) by the given angle. Since the turtle
    /// rotates in place, its position will not change and it will not draw anything while it
    /// turns.
    ///
    /// The `angle` parameter is a floating point number that represents how much you want the
    /// turtle to rotate.
    /// The unit of `angle` is "degrees" by default. You can change that by using the
    /// [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
    /// [`use_radians()`](struct.Turtle.html#method.use_radians) methods.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// // rotate left by 30 degrees
    /// turtle.left(30.0);
    ///
    /// // rotate left by 1 radian (57.2957795 degrees)
    /// turtle.use_radians();
    /// turtle.left(1.0);
    /// // Use PI for precise angles in radians
    /// use std::f64::consts::PI;
    /// // This is the same as turning 45.0 degrees
    /// turtle.left(PI/4.0);
    /// # assert_eq!(
    /// #     (turtle.heading() * 1e5).trunc(),
    /// #     (((90f64 + 30f64).to_radians() + 1.0 + PI/4.0) * 1e5).trunc()
    /// # );
    /// ```
    pub fn left(&mut self, angle: Angle) {
        block_on(self.turtle.left(angle))
    }

    /// Waits for the specified number of seconds before executing the next command.
    ///
    /// ```rust,no_run
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.forward(100.0);
    /// turtle.wait(2.0);
    /// // The turtle will stop for 2 seconds before proceeding to this line
    /// turtle.forward(50.0);
    /// ```
    pub fn wait(&mut self, secs: f64) {
        block_on(self.turtle.wait(secs))
    }

    pub fn arc_left(&mut self, radius: Distance, extent: Angle) {
        block_on(self.turtle.arc_left(radius, extent))
    }

    pub fn arc_right(&mut self, radius: Distance, extent: Angle) {
        block_on(self.turtle.arc_right(radius, extent))
    }

    pub(crate) fn into_async(self) -> AsyncTurtle {
        self.turtle
    }

    /// Returns the current speed of the turtle.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.set_speed(8);
    /// assert_eq!(turtle.speed(), 8);
    /// ```
    ///
    /// See the documentation for the [`Speed` struct](struct.Speed.html) for more information.
    pub fn speed(&self) -> Speed {
        block_on(self.turtle.speed())
    }

    /// Set the turtle's movement and rotation speed to the given value. A higher value will make
    /// the turtle's walking and turning animations faster.
    ///
    /// You can pass either a number or certain strings like `"slow"`, `"normal"`, and `"fast"`.
    /// See the documentation for the [`Speed` struct](struct.Speed.html) for all of the different
    /// options as well as the valid range of numbers that can be used for speeds.
    ///
    /// ```rust,no_run
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.set_speed("normal");
    /// turtle.set_speed("fast");
    /// turtle.set_speed(2);
    /// turtle.set_speed(12);
    /// turtle.set_speed("slower");
    /// // Constructing a Speed directly works too, but the syntax above is often more convenient
    /// turtle.set_speed(Speed::from(2));
    /// ```
    ///
    /// Any invalid string or numeric value outside of the valid range will cause the program to
    /// `panic!` at runtime.
    ///
    /// # Moving Instantly
    ///
    /// Setting the speed to `"instant"` results in no animation. The turtle moves instantly
    /// and turns instantly. This is often used to position the turtle before you start to draw
    /// something. You can set the speed to instant, move the turtle into the position you want to
    /// start your drawing from and then set the speed back to `"normal"`.
    ///
    /// ```rust,no_run
    /// # use turtle::*;
    /// let mut turtle = Turtle::new();
    /// turtle.set_speed("instant");
    /// // Move to a position 300 steps to the left of the start position
    /// turtle.right(90.0);
    /// turtle.backward(300.0);
    ///
    /// // The turtle is in position we want it to start at,
    /// // so let's set the speed back to normal
    /// turtle.set_speed("normal");
    /// // Start drawing from here...
    /// ```
    pub fn set_speed<S: Into<Speed>>(&mut self, speed: S) {
        self.turtle.set_speed(speed)
    }

    /// Returns the turtle's current location (x, y)
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.forward(100.0);
    /// let pos = turtle.position();
    /// assert_eq!(pos.round(), Point {x: 0.0, y: 100.0});
    /// ```
    pub fn position(&self) -> Point {
        block_on(self.turtle.position())
    }

    /// Moves the turtle directly to the given position. See the [`Point` struct](struct.Point.html)
    /// documentation for more information.
    ///
    /// If the pen is down, this will draw a line. The turtle will not turn to face the direction
    /// in which it is moving. It's heading will stay the same.
    /// Use [`set_speed()`](struct.Turtle.html#method.set_speed) to control the animation speed.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// let heading = turtle.heading();
    /// assert_eq!(turtle.position(), Point {x: 0.0, y: 0.0});
    /// turtle.go_to([100.0, -150.0]);
    /// // The heading has not changed, but the turtle has moved to the new position
    /// assert_eq!(turtle.heading(), heading);
    /// assert_eq!(turtle.position(), Point {x: 100.0, y: -150.0});
    /// ```
    pub fn go_to<P: Into<Point>>(&mut self, position: P) {
        block_on(self.turtle.go_to(position))
    }

    /// Goes to the given x-coordinate, keeping the y-coordinate and heading of the turtle the
    /// same. See [`go_to()`](struct.Turtle.html#method.go_to) for more information.
    pub fn set_x(&mut self, x: f64) {
        block_on(self.turtle.set_x(x))
    }

    /// Goes to the given y-coordinate, keeping the x-coordinate and heading of the turtle the
    /// same. See [`go_to()`](struct.Turtle.html#method.go_to) for more information.
    pub fn set_y(&mut self, y: f64) {
        block_on(self.turtle.set_y(y))
    }

    /// Moves instantaneously to the origin and resets the heading to face north.
    ///
    /// ```rust
    /// # use turtle::*;
    /// let mut turtle = Turtle::new();
    /// let start_position = turtle.position().round();
    /// let start_heading = turtle.heading().round();
    /// turtle.right(55.0);
    /// turtle.forward(127.0);
    /// assert_ne!(turtle.position().round(), start_position);
    /// assert_ne!(turtle.heading().round(), start_heading);
    /// turtle.home();
    /// assert_eq!(turtle.position().round(), start_position);
    /// assert_eq!(turtle.heading().round(), start_heading);
    /// ```
    pub fn home(&mut self) {
        block_on(self.turtle.home())
    }

    /// Returns the turtle's current heading.
    ///
    /// The unit of the returned angle is degrees by default, but can be set using the
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
    /// # use turtle::*;
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
    /// ```
    pub fn heading(&self) -> Angle {
        block_on(self.turtle.heading())
    }

    /// Rotate the turtle so that its heading is the given angle.
    ///
    /// The unit of `angle` is degrees by default, but can be set using the
    /// [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
    /// [`use_radians()`](struct.Turtle.html#method.use_radians) methods.
    ///
    /// The turtle will attempt to rotate as little as possible in order to reach the given heading
    /// (between -180 and 179 degrees).
    /// Use [`set_speed()`](struct.Turtle.html#method.set_speed) to control the animation speed.
    ///
    /// Here are some common directions in degrees and radians:
    ///
    /// | Cardinal Direction | Heading (degrees) | Heading (radians) |
    /// | ------------------ | ----------------- | ----------------- |
    /// | East               | 0.0&deg;          | `0.0`             |
    /// | North              | 90.0&deg;         | `PI/2`            |
    /// | West               | 180.0&deg;        | `PI`              |
    /// | South              | 270.0&deg;        | `3*PI/2`          |
    ///
    /// See [`heading()`](struct.Turtle.html#method.heading) for more information.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use turtle::*;
    /// // Turtles start facing north
    /// let mut turtle = Turtle::new();
    /// // The rounding is to account for floating-point error
    /// assert_eq!(turtle.heading().round(), 90.0);
    /// turtle.set_heading(31.0);
    /// assert_eq!(turtle.heading().round(), 31.0);
    /// turtle.set_heading(293.0);
    /// assert_eq!(turtle.heading().round(), 293.0);
    /// turtle.set_heading(1.0);
    /// assert_eq!(turtle.heading().round(), 1.0);
    /// // Angles should not exceed 360.0, even when we set them to values larger than that
    /// turtle.set_heading(367.0);
    /// assert_eq!(turtle.heading().round(), 7.0);
    /// ```
    pub fn set_heading(&mut self, angle: Angle) {
        block_on(self.turtle.set_heading(angle))
    }

    /// Returns true if `Angle` values will be interpreted as degrees.
    ///
    /// See [`use_degrees()`](struct.Turtle.html#method.use_degrees) for more information.
    pub fn is_using_degrees(&self) -> bool {
        self.turtle.is_using_degrees()
    }

    /// Returns true if `Angle` values will be interpreted as radians.
    ///
    /// See [`use_radians()`](struct.Turtle.html#method.use_degrees) for more information.
    pub fn is_using_radians(&self) -> bool {
        self.turtle.is_using_radians()
    }

    /// Change the angle unit to degrees.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// # turtle.use_radians();
    /// assert!(!turtle.is_using_degrees());
    /// turtle.use_degrees();
    /// assert!(turtle.is_using_degrees());
    ///
    /// // This will now be interpreted as 1.0 degree
    /// turtle.right(1.0);
    /// ```
    pub fn use_degrees(&mut self) {
        self.turtle.use_degrees()
    }

    /// Change the angle unit to radians.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// assert!(!turtle.is_using_radians());
    /// turtle.use_radians();
    /// assert!(turtle.is_using_radians());
    ///
    /// // This will now be interpreted as 1.0 radian
    /// turtle.right(1.0);
    /// ```
    pub fn use_radians(&mut self) {
        self.turtle.use_radians()
    }

    /// Return true if pen is down, false if it’s up.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// assert!(turtle.is_pen_down());
    /// turtle.pen_up();
    /// assert!(!turtle.is_pen_down());
    /// turtle.pen_down();
    /// assert!(turtle.is_pen_down());
    /// ```
    pub fn is_pen_down(&self) -> bool {
        block_on(self.turtle.is_pen_down())
    }

    /// Pull the pen down so that the turtle draws while moving.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// # turtle.pen_up();
    /// assert!(!turtle.is_pen_down());
    /// // This will move the turtle, but not draw any lines
    /// turtle.forward(100.0);
    /// turtle.pen_down();
    /// assert!(turtle.is_pen_down());
    /// // The turtle will now draw lines again
    /// turtle.forward(100.0);
    /// ```
    pub fn pen_down(&mut self) {
        self.turtle.pen_down()
    }

    /// Pick the pen up so that the turtle does not draw while moving
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// assert!(turtle.is_pen_down());
    /// // The turtle will move and draw a line
    /// turtle.forward(100.0);
    /// turtle.pen_up();
    /// assert!(!turtle.is_pen_down());
    /// // Now, the turtle will move, but not draw anything
    /// turtle.forward(100.0);
    /// ```
    pub fn pen_up(&mut self) {
        self.turtle.pen_up()
    }

    /// Returns the size (thickness) of the pen. The thickness is measured in pixels.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.set_pen_size(25.0);
    /// assert_eq!(turtle.pen_size(), 25.0);
    /// ```
    ///
    /// See [`set_pen_size()`](struct.Turtle.html#method.set_pen_size) for more details.
    pub fn pen_size(&self) -> f64 {
        block_on(self.turtle.pen_size())
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
    /// ![turtle pen thickness](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/pen_thickness.png)
    ///
    /// Notice that while the turtle travels in a straight line, it produces different thicknesses
    /// of lines which appear like large rectangles.
    pub fn set_pen_size(&mut self, thickness: f64) {
        self.turtle.set_pen_size(thickness)
    }

    /// Returns the color of the pen.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.set_pen_color("blue");
    /// assert_eq!(turtle.pen_color(), "blue".into());
    /// ```
    ///
    /// See the [`Color` struct](struct.Color.html) for more information about colors.
    pub fn pen_color(&self) -> Color {
        block_on(self.turtle.pen_color())
    }

    /// Sets the color of the pen to the given color.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`Color` struct](struct.Color.html) for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use turtle::Drawing;
    ///
    /// fn main() {
    ///     let mut drawing = Drawing::new();
    ///     let mut turtle = drawing.add_turtle();
    ///     drawing.set_background_color("light grey");
    ///     turtle.set_pen_size(3.0);
    ///
    ///     let colors = ["red", "green", "blue"];
    ///
    ///     for i in 0..36 {
    ///         turtle.set_pen_color(colors[i % colors.len()]);
    ///         turtle.forward(25.0);
    ///         turtle.right(10.0);
    ///     }
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle pen color](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/colored_circle.png)
    pub fn set_pen_color<C: Into<Color> + Copy + Debug>(&mut self, color: C) {
        self.turtle.set_pen_color(color)
    }

    /// Returns the current fill color.
    ///
    /// This will be used to fill the shape when
    /// [`begin_fill()`](struct.Turtle.html#method.begin_fill) and
    /// [`end_fill()`](struct.Turtle.html#method.end_fill) are called.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.set_fill_color("coral");
    /// assert_eq!(turtle.fill_color(), "coral".into());
    /// ```
    ///
    /// See the [`Color` struct](struct.Color.html) for more information about colors.
    pub fn fill_color(&self) -> Color {
        block_on(self.turtle.fill_color())
    }

    /// Sets the fill color to the given color.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`Color` struct](struct.Color.html) for more information.
    ///
    /// **Note:** Changing the fill color after calling `begin_fill` will cause the filled shape to
    /// update to the new color.
    ///
    /// # Example
    ///
    /// See [`begin_fill()`](struct.Turtle.html#method.begin_fill) for an example.
    pub fn set_fill_color<C: Into<Color> + Copy + Debug>(&mut self, color: C) {
        self.turtle.set_fill_color(color)
    }

    /// Return true if the turtle is currently filling the shape drawn
    /// by its movements.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// assert!(!turtle.is_filling());
    /// turtle.begin_fill();
    /// assert!(turtle.is_filling());
    /// turtle.end_fill();
    /// assert!(!turtle.is_filling());
    /// ```
    ///
    /// See [`begin_fill()`](struct.Turtle.html#method.begin_fill) for more
    /// information and an example.
    pub fn is_filling(&self) -> bool {
        block_on(self.turtle.is_filling())
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
    /// **Note:** The fill color must be set **before** `begin_fill()` is called in order to be
    /// used when filling the shape.
    ///
    /// ```rust,no_run
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
    /// ![turtle fill example](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/red_circle.png)
    pub fn begin_fill(&mut self) {
        self.turtle.begin_fill()
    }

    /// Stop filling the shape drawn by the turtle's movements.
    ///
    /// **Rule of thumb:** For every call to [`begin_fill()`](struct.Turtle.html#method.begin_fill),
    /// there should be a corresponding call to [`end_fill()`](struct.Turtle.html#method.end_fill).
    ///
    /// See [`begin_fill()`](struct.Turtle.html#method.begin_fill) for more information.
    pub fn end_fill(&mut self) {
        self.turtle.end_fill()
    }

    /// Returns true if the turtle is visible.
    ///
    /// ```rust
    /// # use turtle::*;
    /// let mut turtle = Turtle::new();
    /// assert!(turtle.is_visible());
    /// turtle.hide();
    /// assert!(!turtle.is_visible());
    /// turtle.show();
    /// assert!(turtle.is_visible());
    /// ```
    pub fn is_visible(&self) -> bool {
        block_on(self.turtle.is_visible())
    }

    /// Makes the turtle invisible. The shell will not be shown, but drawings will continue.
    ///
    /// Useful for some complex drawings.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// assert!(turtle.is_visible());
    /// turtle.hide();
    /// assert!(!turtle.is_visible());
    /// ```
    pub fn hide(&mut self) {
        self.turtle.hide()
    }

    /// Makes the turtle visible.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// # turtle.hide();
    /// assert!(!turtle.is_visible());
    /// turtle.show();
    /// assert!(turtle.is_visible());
    /// ```
    pub fn show(&mut self) {
        self.turtle.show()
    }

    /// Delete the turtle's drawings from the screen, re-center the turtle and reset all of the
    /// turtle's state (speed, color, etc.) back to the default.
    ///
    /// ```rust
    /// # use turtle::*;
    /// let mut turtle = Turtle::new();
    ///
    /// turtle.left(43.0);
    /// turtle.forward(289.0);
    /// turtle.set_pen_color("red");
    ///
    /// let position = turtle.position();
    /// let heading = turtle.heading();
    ///
    /// turtle.reset();
    ///
    /// assert_eq!(turtle.heading(), 90.0);
    /// assert_eq!(turtle.position(), Point {x: 0.0, y: 0.0});
    /// assert_ne!(turtle.pen_color(), "red".into());
    /// ```
    pub fn reset(&mut self) {
        self.turtle.reset()
    }

    /// Delete the turtle's drawings from the screen.
    ///
    /// Does not move turtle. Position, speed and heading of the turtle are not affected. The
    /// background color and any other settings (pen color, size, etc.) all remain the same.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.right(32.0);
    ///     turtle.forward(150.0);
    ///
    ///     turtle.wait_for_click();
    ///     turtle.clear();
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle clear before click](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/clear_before_click.png)
    ///
    /// Once you click on the screen, the drawings will be cleared:
    ///
    /// ![turtle clear before click](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/clear_after_click.png)
    pub fn clear(&mut self) {
        self.turtle.clear()
    }

    /// Rotates the turtle to face the given point. See the [`Point` struct](struct.Point.html)
    /// documentation for more information.
    ///
    /// If the coordinates are the same as the turtle's current position, no rotation takes place.
    /// Always rotates the least amount necessary in order to face the given point.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     // moving the turtle to the bottom on the screen in the middle
    ///     turtle.pen_up();
    ///     turtle.go_to([0.0, -300.0]);
    ///     turtle.set_heading(90.0);
    ///     turtle.pen_down();
    ///
    ///     // the turtle will go up following an oscillating point
    ///     let mut i: f64 = 0.0;
    ///     // choosing an arbitrary constant to multiply
    ///     // the cos function, result between -5000 and 5000
    ///     let c = 5000.0;
    ///     // just draw a few full cicles
    ///     while i < 15.0 {
    ///         let f = (i).cos()*c;
    ///         // following the oscillating point above at y=1000
    ///         turtle.turn_towards([f, 1000.0]);
    ///         // going forward for a small amount
    ///         turtle.forward(1.0);
    ///         // incrementing the angle
    ///         i = i + 0.01;
    ///     }
    /// }
    /// ```
    pub fn turn_towards<P: Into<Point>>(&mut self, target: P) {
        block_on(self.turtle.turn_towards(target))
    }

    /// Convenience function that waits for a click to occur before returning.
    ///
    /// Useful for when you want the turtle to wait for the user to click before continuing. Use
    /// this to force the turtle to wait before it starts drawing at the beginning of your program.
    ///
    /// This method uses [`poll_event()`](struct.Drawing.html#method.poll_event) internally and
    /// ignores any other events that take place before the click is received.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.wait_for_click();
    ///     // The turtle will wait for the screen to be clicked before continuing
    ///     turtle.forward(100.0);
    /// }
    /// ```
    pub fn wait_for_click(&mut self) {
        block_on(self.turtle.wait_for_click())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::radians;

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

    #[test]
    fn clear_leaves_position_and_heading() {
        let mut turtle = Turtle::new();
        assert_eq!(turtle.position(), Point::origin());
        assert_eq!(turtle.heading(), 90.0);
        turtle.forward(100.0);
        turtle.set_heading(51.0);
        turtle.clear();
        // The rounding is to account for floating-point error
        assert_eq!(turtle.position().round(), Point { x: 0.0, y: 100.0 });
        assert_eq!(turtle.heading(), 51.0);
    }

    #[test]
    fn turn_towards() {
        let mut turtle = Turtle::new();

        // Turn from each cardinal direction to each cardinal direction
        for n in 0..16 as u32 {
            let original_angle = radians::TWO_PI * n as f64 / 16.0;
            for i in 0..16 as u32 {
                turtle.turn_towards([original_angle.cos(), original_angle.sin()]);
                assert_eq!(turtle.heading().ceil(), original_angle.to_degrees().ceil());

                let target_angle = radians::TWO_PI * i as f64 / 16.0;
                turtle.turn_towards([target_angle.cos(), target_angle.sin()]);
                assert_eq!(turtle.heading().ceil(), target_angle.to_degrees().ceil());
            }
        }
    }

    #[test]
    #[should_panic(expected = "Invalid thickness: -10. The pen thickness must be greater than or equal to zero")]
    fn set_pen_size_rejects_negative() {
        let mut turtle = Turtle::new();
        turtle.set_pen_size(-10.0);
    }

    #[test]
    #[should_panic(expected = "Invalid thickness: NaN. The pen thickness must be greater than or equal to zero")]
    fn set_pen_size_rejects_nan() {
        let mut turtle = Turtle::new();
        turtle.set_pen_size(::std::f64::NAN);
    }

    #[test]
    #[should_panic(expected = "Invalid thickness: inf. The pen thickness must be greater than or equal to zero")]
    fn set_pen_size_rejects_inf() {
        let mut turtle = Turtle::new();
        turtle.set_pen_size(::std::f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "Invalid thickness: -inf. The pen thickness must be greater than or equal to zero")]
    fn set_pen_size_rejects_neg_inf() {
        let mut turtle = Turtle::new();
        turtle.set_pen_size(-::std::f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "Invalid color: Color { red: NaN, green: 0.0, blue: 0.0, alpha: 0.0 }. See the color module documentation for more information.")]
    fn rejects_invalid_pen_color() {
        let mut turtle = Turtle::new();
        turtle.set_pen_color(Color {
            red: ::std::f64::NAN,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        });
    }

    #[test]
    #[should_panic(expected = "Invalid color: Color { red: NaN, green: 0.0, blue: 0.0, alpha: 0.0 }. See the color module documentation for more information.")]
    fn rejects_invalid_fill_color() {
        let mut turtle = Turtle::new();
        turtle.set_fill_color(Color {
            red: ::std::f64::NAN,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        });
    }

    #[test]
    fn ignores_nan_inf_zero() {
        let mut turtle = Turtle::new();

        let default_position = turtle.position();
        let default_heading = turtle.heading();

        turtle.forward(10.0);
        turtle.right(90.0);
        turtle.forward(20.0);

        let position = turtle.position();
        assert_ne!(position, default_position);
        let heading = turtle.heading();
        assert_ne!(heading, default_heading);

        turtle.forward(0.0);
        turtle.forward(::std::f64::NAN);
        turtle.forward(::std::f64::INFINITY);
        turtle.forward(-::std::f64::INFINITY);

        turtle.backward(0.0);
        turtle.backward(::std::f64::NAN);
        turtle.backward(::std::f64::INFINITY);
        turtle.backward(-::std::f64::INFINITY);

        turtle.left(0.0);
        turtle.left(::std::f64::NAN);
        turtle.left(::std::f64::INFINITY);
        turtle.left(-::std::f64::INFINITY);

        turtle.right(0.0);
        turtle.right(::std::f64::NAN);
        turtle.right(::std::f64::INFINITY);
        turtle.right(-::std::f64::INFINITY);

        turtle.wait(0.0);
        turtle.wait(::std::f64::NAN);
        turtle.wait(::std::f64::INFINITY);
        turtle.wait(-::std::f64::INFINITY);

        turtle.arc_left(0.0, 0.0);
        turtle.arc_left(0.0, f64::NAN);
        turtle.arc_left(0.0, f64::INFINITY);
        turtle.arc_left(0.0, -f64::INFINITY);
        turtle.arc_left(f64::NAN, 0.0);
        turtle.arc_left(f64::NAN, f64::NAN);
        turtle.arc_left(f64::NAN, f64::INFINITY);
        turtle.arc_left(f64::NAN, -f64::INFINITY);
        turtle.arc_left(f64::INFINITY, 0.0);
        turtle.arc_left(f64::INFINITY, f64::NAN);
        turtle.arc_left(f64::INFINITY, f64::INFINITY);
        turtle.arc_left(f64::INFINITY, -f64::INFINITY);
        turtle.arc_left(-f64::INFINITY, 0.0);
        turtle.arc_left(-f64::INFINITY, f64::NAN);
        turtle.arc_left(-f64::INFINITY, f64::INFINITY);
        turtle.arc_left(-f64::INFINITY, -f64::INFINITY);

        turtle.arc_right(0.0, 0.0);
        turtle.arc_right(0.0, f64::NAN);
        turtle.arc_right(0.0, f64::INFINITY);
        turtle.arc_right(0.0, -f64::INFINITY);
        turtle.arc_right(f64::NAN, 0.0);
        turtle.arc_right(f64::NAN, f64::NAN);
        turtle.arc_right(f64::NAN, f64::INFINITY);
        turtle.arc_right(f64::NAN, -f64::INFINITY);
        turtle.arc_right(f64::INFINITY, 0.0);
        turtle.arc_right(f64::INFINITY, f64::NAN);
        turtle.arc_right(f64::INFINITY, f64::INFINITY);
        turtle.arc_right(f64::INFINITY, -f64::INFINITY);
        turtle.arc_right(-f64::INFINITY, 0.0);
        turtle.arc_right(-f64::INFINITY, f64::NAN);
        turtle.arc_right(-f64::INFINITY, f64::INFINITY);
        turtle.arc_right(-f64::INFINITY, -f64::INFINITY);

        assert_eq!(turtle.position(), position);
        assert_eq!(turtle.heading(), heading);
    }

    #[test]
    fn ignores_nan_inf() {
        let mut turtle = Turtle::new();

        let default_position = turtle.position();
        let default_heading = turtle.heading();

        turtle.forward(10.0);
        turtle.right(90.0);
        turtle.forward(20.0);

        let position = turtle.position();
        assert_ne!(position, default_position);
        let heading = turtle.heading();
        assert_ne!(heading, default_heading);

        turtle.turn_towards([::std::f64::NAN, 0.0]);
        turtle.turn_towards([0.0, ::std::f64::NAN]);
        turtle.turn_towards([::std::f64::NAN, ::std::f64::NAN]);
        turtle.turn_towards([::std::f64::INFINITY, 0.0]);
        turtle.turn_towards([0.0, ::std::f64::INFINITY]);
        turtle.turn_towards([::std::f64::INFINITY, ::std::f64::INFINITY]);
        turtle.turn_towards([-::std::f64::INFINITY, 0.0]);
        turtle.turn_towards([0.0, -::std::f64::INFINITY]);
        turtle.turn_towards([-::std::f64::INFINITY, -::std::f64::INFINITY]);

        turtle.go_to([::std::f64::NAN, 0.0]);
        turtle.go_to([0.0, ::std::f64::NAN]);
        turtle.go_to([::std::f64::NAN, ::std::f64::NAN]);
        turtle.go_to([::std::f64::INFINITY, 0.0]);
        turtle.go_to([0.0, ::std::f64::INFINITY]);
        turtle.go_to([::std::f64::INFINITY, ::std::f64::INFINITY]);
        turtle.go_to([-::std::f64::INFINITY, 0.0]);
        turtle.go_to([0.0, -::std::f64::INFINITY]);
        turtle.go_to([-::std::f64::INFINITY, -::std::f64::INFINITY]);

        turtle.set_x(::std::f64::NAN);
        turtle.set_x(::std::f64::INFINITY);
        turtle.set_x(-::std::f64::INFINITY);

        turtle.set_y(::std::f64::NAN);
        turtle.set_y(::std::f64::INFINITY);
        turtle.set_y(-::std::f64::INFINITY);

        turtle.set_heading(::std::f64::NAN);
        turtle.set_heading(::std::f64::INFINITY);
        turtle.set_heading(-::std::f64::INFINITY);

        assert_eq!(turtle.position(), position);
        assert_eq!(turtle.heading(), heading);
    }
}
