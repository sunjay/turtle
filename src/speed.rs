use std::f64;
use std::f64::consts::PI;

use rand::{Rand, Rng};

use radians::Radians;
use types::Distance;

/// Represents the supported movement and rotation speeds
///
/// See [`Turtle::set_speed` method](struct.Turtle.html#method.set_speed) for more information.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
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

impl Speed {
    /// Converts a speed to its value as pixels per second
    pub(crate) fn to_absolute(self) -> Distance {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        match self {
            One => 50.,
            Two => 100.,
            Three => 200.,
            Four => 300.,
            Five => 500.,
            Six => 600.,
            Seven => 1000.,
            Eight => 2000.,
            Nine => 3000.,
            Ten => 5000.,
            Instant => f64::INFINITY,
        }
    }

    /// Converts a speed to its value as radians per second
    pub(crate) fn to_rotation(self) -> Radians {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        Radians::from_radians_value(match self {
            One => 0.7 * PI,
            Two => 0.9 * PI,
            Three => 1.0 * PI,
            Four => 2.0 * PI,
            Five =>  3.0 * PI,
            Six => 6.0 * PI,
            Seven => 8.0 * PI,
            Eight => 12.0 * PI,
            Nine => 14.0 * PI,
            Ten => 16.0 * PI,
            Instant => f64::INFINITY,
        })
    }
}

impl Rand for Speed {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        (rng.gen::<i32>() % 10).into()
    }
}

impl<'a> From<&'a str> for Speed {
    fn from(s: &'a str) -> Self {
        use self::Speed::*;

        match s {
            "slowest" => One,
            "slow" => Three,
            "normal" => Six,
            "fast" => Eight,
            "fastest" => Ten,
            "instant" => Instant,
            _ => panic!("Invalid speed specified, use one of the words: 'slowest', 'slow', 'normal', 'fast', 'fastest', 'instant'"),
        }
    }
}

impl From<i32> for Speed {
    fn from(n: i32) -> Self {
        use self::Speed::*;

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
