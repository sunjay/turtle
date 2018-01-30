use std::fmt;
use std::f64;
use std::f64::consts::PI;

use radians::Radians;
use rand::{Rand, Rng};
use {Distance};

const MAX_SPEED: i32 = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum SpeedLevel {
    Value(i32),
    Instant,
}

/// Represents both the movement and rotation speed of the turtle.
///
/// You can create a `Speed` value by converting it from a number of different types. `Speed`
/// values can be compared for equality with `i32` values. This is a little more convenient than
/// converting the `i32` to `Speed` every time you want to make a comparison.
///
/// ```rust
/// # use turtle::Speed;
/// // This value is of type `Speed` and it is converted from an `i32`
/// let speed: Speed = 1.into();
/// // Speed values can be compared to integers
/// assert_eq!(speed, 1);
/// // This is equivalent to the following
/// assert_eq!(speed, Speed::from(1));
/// ```
///
/// There is no need to call `.into()` when passing a speed into the
/// [`Turtle::set_speed` method](struct.Turtle.html#method.set_speed).
///
/// ```rust
/// # use turtle::{Turtle};
/// let mut turtle = Turtle::new();
/// turtle.set_speed(22);
/// ```
///
/// See the [`Turtle::set_speed` method](struct.Turtle.html#method.set_speed) for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Speed(SpeedLevel);

impl Speed {
    /// Returns the speed value that represents instantly moving and rotating with no further
    /// animation
    ///
    /// ```rust
    /// use turtle::{Speed};
    /// assert_eq!(Speed::instant(), 0);
    /// ```
    pub fn instant() -> Self {
        Speed(SpeedLevel::Instant)
    }

    /// Converts a speed to its value as a movement speed in pixels per second
    pub(crate) fn to_movement(self) -> Distance {
        use self::SpeedLevel::*;
        match self.0 {
            Value(speed) => speed as f64 * 500.0,
            Instant => f64::INFINITY,
        }
    }

    /// Converts a speed to its value as radians per second
    pub(crate) fn to_rotation(self) -> Radians {
        use self::SpeedLevel::*;
        Radians::from_radians_value(match self.0 {
            Value(speed) => speed as f64 * (2.0*PI),
            Instant => f64::INFINITY,
        })
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SpeedLevel::*;
        write!(f, "{}", match self.0 {
            Value(speed) => speed,
            Instant => 0,
        })
    }
}

impl Rand for Speed {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        (rng.gen::<i32>() % MAX_SPEED).into()
    }
}

impl<'a> From<&'a str> for Speed {
    fn from(s: &'a str) -> Self {
        use self::SpeedLevel::*;

        Speed(match s {
            "slowest" => Value(1),
            "slower" => Value(5),
            "slow" => Value(8),
            "normal" => Value(10),
            "fast" => Value(12),
            "faster" => Value(15),
            "instant" => Instant,
            _ => panic!("Invalid speed specified, use one of the words: 'slowest', 'slower', 'slow', 'normal', 'fast', 'faster', 'instant'"),
        })
    }
}

impl PartialEq<i32> for Speed {
    fn eq(&self, other: &i32) -> bool {
        self.eq(&Speed::from(*other))
    }
}

impl From<i32> for Speed {
    fn from(n: i32) -> Self {
        use self::SpeedLevel::*;

        Speed(match n {
            0 => Instant,
            n if n <= MAX_SPEED => Value(n),
            n => panic!("Invalid speed: {}. Must be a value between 0 and {}", n, MAX_SPEED),
        })
    }
}

// We include this implementation because we use f64 in the rest of the library.
// It makes sense to implement this so that they don't get messed up if they accidentally use a
// floating point literal with set_speed.
impl From<f64> for Speed {
    fn from(n: f64) -> Self {
        (n.round() as i32).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Turtle};

    #[test]
    fn display() {
        for value in 0..MAX_SPEED {
            let speed: Speed = value.into();
            assert_eq!(format!("{}", speed), value.to_string());
        }
    }

    #[test]
    fn speed_strings() {
        let mut turtle = Turtle::new();
        turtle.set_speed("slowest");
        assert_eq!(turtle.speed(), 1);
        turtle.set_speed("slower");
        assert_eq!(turtle.speed(), 5);
        turtle.set_speed("slow");
        assert_eq!(turtle.speed(), 8);
        turtle.set_speed("normal");
        assert_eq!(turtle.speed(), 10);
        turtle.set_speed("fast");
        assert_eq!(turtle.speed(), 12);
        turtle.set_speed("faster");
        assert_eq!(turtle.speed(), 15);
        turtle.set_speed("instant");
        assert_eq!(turtle.speed(), 0);
    }

    #[test]
    #[should_panic(expected = "Invalid speed specified, use one of the words: 'slowest', 'slower', 'slow', 'normal', 'fast', 'faster', 'instant'")]
    fn invalid_speed() {
        let mut turtle = Turtle::new();
        turtle.set_speed("wrong");
    }

    #[test]
    fn speed_values() {
        let mut turtle = Turtle::new();
        for speed in 0..MAX_SPEED {
            turtle.set_speed(speed);
            assert_eq!(turtle.speed(), speed);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid speed: 26. Must be a value between 0 and 25")]
    fn speed_value_out_of_range() {
        let mut turtle = Turtle::new();
        turtle.set_speed(26);
        assert_eq!(turtle.speed(), 0);
    }

    #[test]
    #[should_panic(expected = "Invalid speed: 20394. Must be a value between 0 and 25")]
    fn speed_value_out_of_range2() {
        let mut turtle = Turtle::new();
        turtle.set_speed(20394);
        assert_eq!(turtle.speed(), 0);
    }

    #[test]
    fn speed_values_f64() {
        let mut turtle = Turtle::new();
        for speed in 0..MAX_SPEED {
            turtle.set_speed(speed as f64 + 0.4);
            assert_eq!(turtle.speed(), speed);
        }
    }
}
