use std::env;

/// This type represents and enforces the various speeds allowed
/// for use with the turtle.
///
/// See [`Turtle::set_speed` method](struct.Turtle.html#method.set_speed) for more information.
pub enum Speed {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Instant,
}

impl<'a> From<&'a str> for Speed {
    fn from(s: &'a str) -> Self {
        use Speed::*;

        match s {
            "slowest" => One,
            "slow" => Three,
            "normal" => Six,
            "fast" => Ten,
            "fastest" => Instant,
            _ => panic!("Invalid speed specified, use one of the words: 'slowest', 'slow', 'normal', 'fast', 'fastest'"),
        }
    }
}

impl From<i32> for Speed {
    fn from(n: i32) -> Self {
        use Speed::*;

        match n {
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            10 => Ten,
            _ => Instant,
        }
    }
}

/// This type represents any distance value
pub type Distance = i32;

enum OutputMode {
    /// Actually spawn a window and draw the turtle.
    /// For when the program is running from a terminal.
    Draw,
    /// Only print the drawing commands, but don't spawn
    /// any windows or anything.
    /// For when the program is running in the Turtle IDE
    Print,
}

/// A turtle with a pen attached to its tail
pub struct Turtle {
    mode: OutputMode,
    speed: Speed,
}

impl Turtle {
    /// Initialize a new Turtle instance
    pub fn new() -> Turtle {
        // Attempt to automatically detect if this is running within the Turtle IDE
        let mode = match env::args().any(|a| a == "--turtle-ide-print-mode") {
            false => OutputMode::Draw,
            true => OutputMode::Print,
        };

        Turtle {
            mode,
            speed: "normal".into(),
        }
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
