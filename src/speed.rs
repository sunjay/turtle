use std::f64;
use std::f64::consts::PI;

/// This type represents and enforces the various speeds allowed
/// for use with the turtle.
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
    pub fn to_absolute(self) -> f64 {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        match self {
            One => 10.,
            Two => 20.,
            Three => 30.,
            Four => 40.,
            Five => 50.,
            Six => 60.,
            Seven => 70.,
            Eight => 80.,
            Nine => 90.,
            Ten => 100.,
            Instant => f64::INFINITY,
        }
    }

    /// Converts a speed to its value as radians per second
    //TODO: Return Radians
    pub fn to_rotation(self) -> f64 {
        use self::Speed::*;
        // Arbitrary values that can be tweaked
        // Just make sure to keep invariants like Five > Three, etc.
        match self {
            One => PI / 16.,
            Two => PI / 14.,
            Three => PI / 12.,
            Four => PI / 10.,
            Five =>  PI / 8.,
            Six => PI / 6.,
            Seven => PI / 4.,
            Eight => PI / 3.,
            Nine => PI / 2.,
            Ten => PI / 1.,
            Instant => f64::MAX,
        }
    }
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
