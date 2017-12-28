use std::fmt;
use std::f64;
use std::f64::consts::PI;

use radians::Radians;
use rand::{Rand, Rng};
use {Distance};

const MAX_SPEED: u8 = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum SpeedLevel {
    Value(u8),
    Instant,
}

/// Represents the supported movement and rotation speeds
///
/// See [`Turtle::set_speed` method](struct.Turtle.html#method.set_speed) for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Speed(SpeedLevel);

impl Speed {
    /// Converts a speed to its value as a movement speed in pixels per second
    pub fn to_movement(self) -> Distance {
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
        (rng.gen::<i32>() % MAX_SPEED as i32).into()
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

impl From<i32> for Speed {
    fn from(n: i32) -> Self {
        use self::SpeedLevel::*;

        Speed(match n {
            0 => Instant,
            n if n as u8 <= MAX_SPEED => Value(n as u8),
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
    fn speed_names() {
        let mut turtle = Turtle::new();
        turtle.set_speed("slowest");
        assert_eq!(turtle.speed(), Speed::One);
        turtle.set_speed("slow");
        assert_eq!(turtle.speed(), Speed::Three);
        turtle.set_speed("normal");
        assert_eq!(turtle.speed(), Speed::Six);
        turtle.set_speed("fast");
        assert_eq!(turtle.speed(), Speed::Eight);
        turtle.set_speed("fastest");
        assert_eq!(turtle.speed(), Speed::Ten);
        turtle.set_speed("instant");
        assert_eq!(turtle.speed(), Speed::Instant);
    }

    #[test]
    #[should_panic(expected = "Invalid speed specified, use one of the words: 'slowest', 'slow', 'normal', 'fast', 'fastest', 'instant'")]
    fn invalid_speed() {
        let mut turtle = Turtle::new();
        turtle.set_speed("wrong");
    }

    #[test]
    fn speed_values() {
        let mut turtle = Turtle::new();
        turtle.set_speed(0);
        assert_eq!(turtle.speed(), Speed::Instant);
        turtle.set_speed(1);
        assert_eq!(turtle.speed(), Speed::One);
        turtle.set_speed(2);
        assert_eq!(turtle.speed(), Speed::Two);
        turtle.set_speed(3);
        assert_eq!(turtle.speed(), Speed::Three);
        turtle.set_speed(4);
        assert_eq!(turtle.speed(), Speed::Four);
        turtle.set_speed(5);
        assert_eq!(turtle.speed(), Speed::Five);
        turtle.set_speed(6);
        assert_eq!(turtle.speed(), Speed::Six);
        turtle.set_speed(7);
        assert_eq!(turtle.speed(), Speed::Seven);
        turtle.set_speed(8);
        assert_eq!(turtle.speed(), Speed::Eight);
        turtle.set_speed(9);
        assert_eq!(turtle.speed(), Speed::Nine);
        turtle.set_speed(10);
        assert_eq!(turtle.speed(), Speed::Ten);
        turtle.set_speed(11);
        assert_eq!(turtle.speed(), Speed::Instant);
        turtle.set_speed(20394);
        assert_eq!(turtle.speed(), Speed::Instant);
    }

    #[test]
    fn speed_values_f64() {
        let mut turtle = Turtle::new();
        turtle.set_speed(0.4);
        assert_eq!(turtle.speed(), Speed::Instant);
        turtle.set_speed(1.4);
        assert_eq!(turtle.speed(), Speed::One);
        turtle.set_speed(2.4);
        assert_eq!(turtle.speed(), Speed::Two);
        turtle.set_speed(3.4);
        assert_eq!(turtle.speed(), Speed::Three);
        turtle.set_speed(4.4);
        assert_eq!(turtle.speed(), Speed::Four);
        turtle.set_speed(5.4);
        assert_eq!(turtle.speed(), Speed::Five);
        turtle.set_speed(6.4);
        assert_eq!(turtle.speed(), Speed::Six);
        turtle.set_speed(7.4);
        assert_eq!(turtle.speed(), Speed::Seven);
        turtle.set_speed(8.4);
        assert_eq!(turtle.speed(), Speed::Eight);
        turtle.set_speed(9.4);
        assert_eq!(turtle.speed(), Speed::Nine);
        turtle.set_speed(10.4);
        assert_eq!(turtle.speed(), Speed::Ten);
        turtle.set_speed(11.4);
        assert_eq!(turtle.speed(), Speed::Instant);
        turtle.set_speed(20394.4);
        assert_eq!(turtle.speed(), Speed::Instant);
    }
}
