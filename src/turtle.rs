use std::env;

use screen::{Screen, Pen};
use speed::Speed;
use point::Point;
use ideoutput::IDEOutput;

/// This type represents any distance value
pub type Distance = i32;

/// A turtle with a pen attached to its tail
pub struct Turtle {
    screen: Box<Screen>,
    speed: Speed,
    position: Point,
    settings: Pen,
}

impl Turtle {
    /// Initialize a new Turtle instance
    pub fn new() -> Turtle {
        let screen: Box<Screen> = match env::args().any(|a| a == "--turtle-ide-print-mode") {
            false => unimplemented!(),
            true => Box::new(IDEOutput::new()),
        };
        Turtle {
            // Attempt to automatically detect if this is running within the Turtle IDE
            screen: screen,
            speed: "normal".into(),
            position: Point {x: 0, y: 0},
            settings: Pen::default(),
        }
    }

    /// Returns the current speed of the turtle
    fn speed(&self) -> Speed {
        self.speed
    }

    /// Set the turtle's speed to the given setting.
    ///
    /// Usually this method is used as shown below:
    ///
    /// ```rust,no_run
    /// # extern crate turtleide;
    /// # fn main() {
    /// # let mut turtle = turtleide::Turtle::new();
    /// turtle.set_speed("normal".into());
    /// turtle.set_speed("fast".into());
    /// turtle.set_speed(2.into());
    /// turtle.set_speed(10.into());
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
    /// directly using `Speed::Six`, you usually use `6.into()`. This is the
    /// same as using `Speed::from(6)` but much more compact. This works because
    /// any type that implements the `From` trait gets a matching implementation
    /// of the `Into` trait.
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// Move the turtle forward by the given amount of `distance`.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move backward
    /// using this method.
    pub fn forward(&mut self, distance: Distance) {
    }
}
