use std::thread;
use std::time::Duration;

use radians::{self, Radians};
use turtle_window::TurtleWindow;
use event::MouseButton;
use {Speed, Color, Event, Point, Distance, Angle};

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

/// A turtle with a pen attached to its tail
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

impl Default for Turtle {
    fn default() -> Self {
        Self::new()
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
    /// extern crate turtle;
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
        Turtle {
            window: TurtleWindow::new(),
            angle_unit: AngleUnit::Degrees,
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
    /// # #![allow(unused_variables, unused_mut)]
    /// # extern crate turtle;
    /// # use turtle::Turtle;
    /// # fn main() {
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
    /// # assert_eq!(turtle.position()[1].round(), -113.0);
    /// # }
    /// ```
    pub fn forward(&mut self, distance: Distance) {
        self.window.forward(distance);
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
    /// # #![allow(unused_variables, unused_mut)]
    /// # extern crate turtle;
    /// # use turtle::Turtle;
    /// # fn main() {
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
    /// # assert_eq!(turtle.position()[1].round(), 69.0);
    /// # }
    /// ```
    pub fn backward(&mut self, distance: Distance) {
        // Moving backwards is essentially moving forwards with a negative distance
        self.window.forward(-distance);
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
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
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
    /// # }
    /// ```
    pub fn right(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, true);
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
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
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
    /// # }
    /// ```
    pub fn left(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, false);
    }

    /// Waits for the specified number of seconds before executing the next command.
    ///
    /// ```rust,no_run
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.forward(100.0);
    /// turtle.wait(2.0);
    /// // The turtle will stop for 2 seconds before proceeding to this line
    /// turtle.forward(50.0);
    /// # }
    /// ```
    pub fn wait(&mut self, secs: f64) {
        thread::sleep(Duration::from_millis((secs * 1000.0) as u64));
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
        //self.window.turtle_mut().speed = speed.into();
        unimplemented!();
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

    /// Moves the turtle directly to the given position.
    ///
    /// If the pen is down, this will draw a line. The turtle will not turn to face the direction
    /// in which it is moving. It's heading will stay the same.
    /// Use [`set_speed()`](struct.Turtle.html#method.set_speed) to control the animation speed.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// let heading = turtle.heading();
    /// assert_eq!(turtle.position(), [0.0, 0.0]);
    /// turtle.go_to([100.0, -150.0]);
    /// // The heading has not changed, but the turtle has moved to the new position
    /// assert_eq!(turtle.heading(), heading);
    /// assert_eq!(turtle.position(), [100.0, -150.0]);
    /// # }
    /// ```
    pub fn go_to(&mut self, position: Point) {
        self.window.go_to(position);
    }

    /// Goes to the given x-coordinate, keeping the y-coordinate and heading of the turtle the
    /// same. See [`go_to()`](struct.Turtle.html#method.go_to) for more information.
    pub fn set_x(&mut self, x: f64) {
        let y = self.position()[1];
        self.go_to([x, y]);
    }

    /// Goes to the given y-coordinate, keeping the x-coordinate and heading of the turtle the
    /// same. See [`go_to()`](struct.Turtle.html#method.go_to) for more information.
    pub fn set_y(&mut self, y: f64) {
        let x = self.position()[0];
        self.go_to([x, y]);
    }

    /// Moves instantaneously to the origin and resets the heading to face north.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// let mut turtle = Turtle::new();
    /// let start_position = turtle.position();
    /// let start_heading = turtle.heading().round();
    /// turtle.right(55.0);
    /// turtle.forward(127.0);
    /// assert_ne!(turtle.heading().round(), start_heading);
    /// assert_ne!(turtle.position()[0].round(), start_position[0].round());
    /// assert_ne!(turtle.position()[1].round(), start_position[1].round());
    /// turtle.home();
    /// assert_eq!(turtle.heading().round(), start_heading);
    /// assert_eq!(turtle.position()[0].round(), start_position[0].round());
    /// assert_eq!(turtle.position()[1].round(), start_position[1].round());
    /// # }
    /// ```
    pub fn home(&mut self) {
        //let mut turtle = self.window.turtle_mut();
        //turtle.position = [0.0, 0.0];
        //turtle.heading = radians::PI/2.0;
        unimplemented!();
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
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
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
    /// # }
    /// ```
    pub fn set_heading(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        let heading = self.window.turtle().heading;
        // Find the amount we need to turn to reach the target heading based on our current heading
        let angle = angle - heading;
        // Normalize the angle to be between -180 and 179 so that we rotate as little as possible
        // Formula from: https://stackoverflow.com/a/24234924/551904
        let angle = angle - radians::TWO_PI * ((angle + radians::PI) / radians::TWO_PI).floor();
        self.window.rotate(angle, false);
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
    ///
    /// // This will now be interpreted as 1.0 degree
    /// turtle.right(1.0);
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
    ///
    /// // This will now be interpreted as 1.0 radian
    /// turtle.right(1.0);
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
        self.window.turtle().pen.enabled
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
        //self.window.turtle_mut().pen.enabled = true;
        unimplemented!();
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
        //self.window.turtle_mut().pen.enabled = false;
        unimplemented!();
    }

    /// Returns the size (thickness) of the pen. The thickness is measured in pixels.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.set_pen_size(25.0);
    /// assert_eq!(turtle.pen_size(), 25.0);
    /// # }
    /// ```
    ///
    /// See [`set_pen_size()`](struct.Turtle.html#method.set_pen_size) for more details.
    pub fn pen_size(&self) -> f64 {
        self.window.turtle().pen.thickness
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
        //self.window.turtle_mut().pen.thickness = thickness;
        unimplemented!();
    }

    /// Returns the color of the pen.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.set_pen_color("blue");
    /// assert_eq!(turtle.pen_color(), "blue".into());
    /// # }
    /// ```
    ///
    /// See the [`color` module](color/index.html) for more information about colors.
    pub fn pen_color(&self) -> Color {
        self.window.turtle().pen.color
    }

    /// Sets the color of the pen to the given color.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`color` module](color/index.html) for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.set_background_color("light grey");
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
    /// ![turtle pen color](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/colored_circle.png)
    pub fn set_pen_color<C: Into<Color>>(&mut self, color: C) {
        //self.window.turtle_mut().pen.color = color.into();
        unimplemented!();
    }

    /// Returns the color of the background.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.set_background_color("purple");
    /// assert_eq!(turtle.background_color(), "purple".into());
    /// # }
    /// ```
    ///
    /// See the [`color` module](color/index.html) for more information about colors.
    pub fn background_color(&self) -> Color {
        self.window.drawing().background
    }

    /// Sets the color of the background to the given color.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`color` module](color/index.html) for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.set_background_color("orange");
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle background](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/orange_background.png)
    pub fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        //self.window.drawing_mut().background = color.into();
        unimplemented!();
    }

    /// Returns the current fill color.
    ///
    /// This will be used to fill the shape when
    /// [`begin_fill()`](struct.Turtle.html#method.begin_fill) and
    /// [`end_fill()`](struct.Turtle.html#method.end_fill) are called.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.set_fill_color("coral");
    /// assert_eq!(turtle.fill_color(), "coral".into());
    /// # }
    /// ```
    ///
    /// See the [`color` module](color/index.html) for more information about colors.
    pub fn fill_color(&self) -> Color {
        self.window.turtle().fill_color
    }

    /// Sets the fill color to the given color.
    ///
    /// **Note:** The fill color must be set **before** `begin_fill()` is called in order to be
    /// used when filling the shape.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`color` module](color/index.html) for more information.
    ///
    /// # Example
    ///
    /// See [`begin_fill()`](struct.Turtle.html#method.begin_fill) for an example.
    pub fn set_fill_color<C: Into<Color>>(&mut self, color: C) {
        //self.window.turtle_mut().fill_color = color.into();
        unimplemented!();
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
    /// extern crate turtle;
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
        //self.window.turtle_mut().visible = false;
        unimplemented!();
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
        //self.window.turtle_mut().visible = true;
        unimplemented!();
    }

    /// Delete the turtle's drawings from the screen, re-center the turtle and reset all of the
    /// turtle's state (speed, color, etc.) back to the default.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.left(43.0);
    /// turtle.forward(289.0);
    /// turtle.set_pen_color("red");
    /// turtle.set_background_color("green");
    /// let position = turtle.position();
    /// let heading = turtle.heading();
    /// turtle.reset();
    /// assert_eq!(turtle.heading(), 90.0);
    /// assert_eq!(turtle.position(), [0.0, 0.0]);
    /// assert_ne!(turtle.pen_color(), "red".into());
    /// assert_ne!(turtle.background_color(), "green".into());
    /// # }
    /// ```
    pub fn reset(&mut self) {
        self.clear();
        //*self.window.turtle_mut() = Default::default();
        //*self.window.drawing_mut() = Default::default();
        unimplemented!();
    }

    /// Delete the turtle's drawings from the screen.
    ///
    /// Does not move turtle. Position, speed and heading of the turtle are not affected. The
    /// background color and any other settings (pen color, size, etc.) all remain the same.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
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
    /// ![turtle clear before click](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/clear_before_click.png)
    ///
    /// Once you click on the screen, the drawings will be cleared:
    ///
    /// ![turtle clear before click](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/clear_after_click.png)
    pub fn clear(&mut self) {
        self.window.clear();
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

    /// Convenience function that waits for a click to occur before returning.
    ///
    /// Useful for when you want your program to wait for the user to click before continuing so
    /// that it doesn't start right away.
    ///
    /// This method uses [`poll_event()`](struct.Turtle.html#method.poll_event) internally and
    /// ignores any other events that take place before the click is received.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
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
        loop {
            if let Some(Event::MouseButtonReleased(MouseButton::Left)) = self.poll_event() {
                break;
            }
        }
    }

    /// Returns the next event (if any). Returns `None` if there are no events to be processed at the
    /// current moment. This **does not** mean that there will never be events later on as the
    /// application continues to run.
    ///
    /// See the [`Event` enum](event/enum.Event.html) for the complete list of events that you can
    /// handle in Turtle.
    ///
    /// # Example
    ///
    /// To use this advanced method, you need to create what is known as an "event loop". An "event
    /// loop" is any loop that handles the events generated by the application. The reason that it
    /// is important to create a loop like this is because events in Turtle are "polled". That
    /// means that every time an event happens, it is placed in a queue (a list) until you ask to
    /// look at it. If you do not check for events continuously, there is a chance that the events
    /// you ask for from `poll_event()` will be outdated.
    ///
    /// Even if you do not use every kind of event, you should aim to poll events using this method
    /// until there are none left to poll. If you do not poll events for a significant amount of
    /// time during your application, favor the events that come later as you poll since those will
    /// be the most recent. This can happen if you run many animations between loop iterations.
    ///
    /// See the [`examples/`](https://github.com/sunjay/turtle/raw/master/examples) directory in
    /// the source code of this library for more examples of how to use events.
    ///
    /// The following is an example of a basic event loop. Notice that it uses two loops. One to
    /// move the turtle continuously, and another to handle all the events available at a given
    /// moment. If it suits your purposes, you may also just use a single loop to handle events
    /// and move the turtle from within that loop. This example is of a more complex case where
    /// it really matters that the most recent information is taken into consideration before any
    /// further movements take place.
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    ///
    /// use turtle::Turtle;
    /// use turtle::event::Key::{Left, Right};
    /// use turtle::Event::KeyPressed;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///
    ///     loop {
    ///         turtle.forward(1.0);
    ///
    ///         while let Some(event) = turtle.poll_event() {
    ///             match event {
    ///                 KeyPressed(key) => match key {
    ///                     Left => {
    ///                         turtle.set_speed(8);
    ///                         for _ in 0..20 {
    ///                             turtle.forward(1.0);
    ///                             turtle.left(4.5);
    ///                         }
    ///                         turtle.set_speed(4);
    ///                     },
    ///                     Right => {
    ///                         turtle.set_speed(8);
    ///                         for _ in 0..20 {
    ///                             turtle.forward(1.0);
    ///                             turtle.right(4.5);
    ///                         }
    ///                         turtle.set_speed(4);
    ///                     },
    ///                     _ => {},
    ///                 },
    ///                 _ => {},
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn poll_event(&mut self) -> Option<Event> {
        self.window.poll_event()
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

    #[test]
    fn clear_leaves_position_and_heading() {
        let mut turtle = Turtle::new();
        assert_eq!(turtle.position(), [0.0, 0.0]);
        assert_eq!(turtle.heading(), 90.0);
        turtle.forward(100.0);
        turtle.set_heading(51.0);
        turtle.clear();
        // The rounding is to account for floating-point error
        assert_eq!(turtle.position()[0].round(), 0.0);
        assert_eq!(turtle.position()[1].round(), 100.0);
        assert_eq!(turtle.heading(), 51.0);
    }
}
