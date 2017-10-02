use std::f64;
use std::f64::consts::PI;

use rand::{Rand, Rng};

use radians::Radians;
use {Distance};

/// Represents the various supported speeds that the turtle can move at
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
    pub fn to_absolute(self) -> Distance {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        match self {
            One => 50.,
            Two => 100.,
            Three => 150.,
            Four => 200.,
            Five => 250.,
            Six => 300.,
            Seven => 350.,
            Eight => 400.,
            Nine => 450.,
            Ten => 500.,
            Instant => f64::INFINITY,
        }
    }

    /// Converts a speed to its value as radians per second
    pub fn to_rotation(self) -> Radians {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        Radians::from_radians_value(match self {
            One => 0.7 * PI,
            Two => 0.9 * PI,
            Three => 1.1 * PI,
            Four => 1.3 * PI,
            Five =>  1.5 * PI,
            Six => 1.8 * PI,
            Seven => 2.1 * PI,
            Eight => 2.4 * PI,
            Nine => 2.7 * PI,
            Ten => 3.0 * PI,
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
        use Speed::*;

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
