/// This type represents and enforces the various speeds allowed
/// for use with the turtle.
///
/// See [`Turtle::set_speed` method](struct.Turtle.html#method.set_speed) for more information.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
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
