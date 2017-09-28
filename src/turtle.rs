use speed::Speed;
use radians::Radians;
use turtle_window::TurtleWindow;

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

/// A turtle with a pen attached to its tail
pub struct Turtle {
    window: TurtleWindow,
    angle_unit: AngleUnit,
}

impl Turtle {
    /// Initialize a new Turtle instance
    pub fn new() -> Turtle {
        Turtle {
            window: TurtleWindow::new(),
            angle_unit: AngleUnit::Degrees,
        }
    }

    /// Returns the current speed of the turtle
    pub fn speed(&self) -> Speed {
        //TODO: Read from rendering thread
        unimplemented!();
    }

    /// Returns the turtle's current location (x, y)
    pub fn position(&self) -> Point {
        //TODO: Read from rendering thread
        unimplemented!();
    }

    /// Returns the turtle's current heading
    pub fn heading(&self) -> Angle {
        //TODO: Read from rendering thread
        unimplemented!();
    }

    /// Returns true if Angle values will be interpreted as degrees
    pub fn is_using_degrees(&self) -> bool {
        self.angle_unit == AngleUnit::Degrees
    }

    /// Returns true if Angle values will be interpreted as radians
    pub fn is_using_radians(&self) -> bool {
        self.angle_unit == AngleUnit::Radians
    }

    /// Return true if pen is down, false if it’s up.
    pub fn is_pen_down(&self) -> bool {
        //TODO: Read from rendering thread
        unimplemented!();
    }

    /// Pull the pen down so that the turtle draws while moving
    pub fn pen_down(&mut self) {
        //TODO: Write to shared data from rendering thread
        unimplemented!();
    }

    /// Pick the pen up so that the turtle does not draw while moving
    pub fn pen_up(&mut self) {
        //TODO: Write to shared data from rendering thread
        unimplemented!();
    }

    /// Set the turtle's speed to the given setting.
    ///
    /// Usually this method is used as shown below:
    ///
    /// ```rust,no_run
    /// # extern crate turtle;
    /// # fn main() {
    /// # let mut turtle = turtle::Turtle::new();
    /// turtle.set_speed("normal");
    /// turtle.set_speed("fast");
    /// turtle.set_speed(2);
    /// turtle.set_speed(10);
    /// # }
    /// ```
    ///
    /// If input is a number greater than 10 or smaller than 1,
    /// speed is set to 0 (Speed::Instant). Strings are converted as follows:
    ///
    /// * "slowest" => Speed::One
    /// * "slow" => Speed::Three
    /// * "normal" => Speed::Six
    /// * "fast" => Speed::Ten
    /// * "fastest" => Speed::Instant
    /// * anything else will cause the program to `panic!` at runtime
    ///
    /// Speeds from 1 to 10 enforce increasingly faster animation of
    /// line drawing and turtle turning.
    ///
    /// Note: speed = 0 means that no animation takes place. forward/back
    /// makes turtle jump and likewise left/right make the turtle turn instantly.
    ///
    /// Using this type is an excellent way to learn about conversion
    /// traits `From` and `Into`. Rather than instantiating `Speed`
    /// directly using `Speed::Six`, you usually use just `6`. This is the
    /// same as using `Speed::from(6)` but much more compact. This works because
    /// any type that implements the `From` trait gets a matching implementation
    /// of the `Into` trait.
    pub fn set_speed<S: Into<Speed>>(&mut self, speed: S) {
        //TODO: Send proper command
        //use speed.into();
        unimplemented!();
    }

    /// Change the angle unit to degrees.
    pub fn use_degrees(&mut self) {
        self.angle_unit = AngleUnit::Degrees;
    }

    /// Change the angle unit to radians.
    pub fn use_radians(&mut self) {
        self.angle_unit = AngleUnit::Radians;
    }

    /// Move the turtle forward by the given amount of `distance`.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move backward
    /// using this method.
    pub fn forward(&mut self, distance: Distance) {
        self.window.forward(distance);
    }

    /// Move the turtle backward by the given amount of `distance`.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move forwards
    /// using this method.
    pub fn backward(&mut self, distance: Distance) {
        // Moving backwards is essentially moving forwards with a negative distance
        self.window.forward(-distance);
    }

    /// Rotate the turtle right (clockwise) by the given angle.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`Turtle::use_degrees`](struct.Turtle.html#method.use_degrees) or
    /// [`Turtle::use_radians`](struct.Turtle.html#method.use_radians).
    pub fn right(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, true);
    }

    /// Rotate the turtle left (counterclockwise) by the given angle.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`Turtle::use_degrees`](struct.Turtle.html#method.use_degrees) or
    /// [`Turtle::use_radians`](struct.Turtle.html#method.use_radians).
    pub fn left(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, false);
    }
}
