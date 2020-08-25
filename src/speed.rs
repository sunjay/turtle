use std::cmp::Ordering;
use std::fmt;

use serde::{Serialize, Deserialize};
use interpolation::lerp;

use crate::radians::{self, Radians};
use crate::rand::{Random, RandomRange};
use crate::Distance;

const MIN_SPEED: i32 = 1;
const MAX_SPEED: i32 = 25;

#[derive(Clone, Copy, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub(crate) enum SpeedLevel {
    Value(i32),
    Instant,
}

impl PartialOrd for SpeedLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use SpeedLevel::*;
        match (*self, *other) {
            (Value(value), Value(ref other_value)) => value.partial_cmp(other_value),
            (Instant, Instant) => Some(Ordering::Equal),
            (Value(_), Instant) => Some(Ordering::Less),
            (Instant, Value(_)) => Some(Ordering::Greater),
        }
    }
}

/// Represents both the movement and rotation speed of the turtle.
///
/// # Creating Speeds
///
/// You can create a `Speed` value by converting either strings or numbers.
///
/// ```rust
/// # use turtle::Speed;
/// // This value is of type `Speed` and it is converted from an `i32`
/// let speed: Speed = 1.into();
///
/// // This value is of type `Speed` and it is converted from a `&str`
/// let slowest_speed: Speed = "slowest".into();
/// # assert_eq!(speed, slowest_speed);
/// ```
///
/// There is no need to call `.into()` when passing a speed into the [`set_speed` method].
/// See the [`set_speed` method] for more information about how that works.
///
/// ```rust
/// # use turtle::{Turtle};
/// let mut turtle = Turtle::new();
/// turtle.set_speed(22); // Same as `turtle.set_speed(22.into())`
/// ```
///
/// **The minimum speed value is 1 and the maximum speed value (currently) is 25.**
///
/// Trying to set the speed to a value out of that range will cause a panic.
///
/// ```rust,should_panic
/// # use turtle::{Turtle};
/// let mut turtle = Turtle::new();
/// turtle.set_speed(26); // panic!
/// ```
///
/// While the minimum speed will not change, the maximum speed may grow larger if the need arises.
/// That is why we chose to panic for invalid speeds instead of defaulting to another value.
///
/// ### String Conversion
///
/// Strings are converted as follows:
///
/// | String      | Value |
/// | ----------- | ----- |
/// | `"slowest"` | `1`   |
/// | `"slower"`  | `5`   |
/// | `"slow"`    | `8`   |
/// | `"normal"`  | `10`  |
/// | `"fast"`    | `12`  |
/// | `"faster"`  | `15`  |
/// | `"instant"` | [see below](#instant) |
///
/// You can use strings to create `Speed` values in the same way numbers were used above. Each of
/// the following is an equivalent way to set the speed to `5`:
///
/// ```rust
/// # use turtle::{Turtle, Speed};
/// # let mut turtle = Turtle::new();
/// turtle.set_speed(5);
/// turtle.set_speed("slower");
/// turtle.set_speed(Speed::from(5)); // Not recommended!
/// turtle.set_speed(Speed::from("slower")); // Not recommended!
/// ```
///
/// # Instant
///
/// There is one special speed value `"instant"` which makes it so that movement and rotation
/// are not animated at all. Instead, the turtle immediately moves and rotates to the position that
/// you directed it to. It will still draw along the way if its pen is down.
///
/// ```rust
/// # use turtle::{Turtle};
/// let mut turtle = Turtle::new();
/// turtle.set_speed("instant");
/// turtle.forward(100.0); // A line will be drawn instantly!
/// ```
///
/// # Comparing Speed Values
///
/// `Speed` values can be compared for equality with `i32` values. This is a little more convenient
/// than converting the `i32` to `Speed` every time you want to make a comparison.
///
/// ```rust
/// # use turtle::Speed;
/// let speed: Speed = 1.into();
/// // Speed values can be compared to integers
/// assert_eq!(speed, 1);
/// // This is equivalent to the following
/// assert_eq!(speed, Speed::from(1));
///
/// // This value is of type `Speed` and it is converted from a `&str`
/// let speed: Speed = "slowest".into();
/// // Speed values can be compared to other speed values
/// assert_eq!(speed, Speed::from("slowest"));
/// // This is equivalent to the following since the slowest speed is 1
/// assert_eq!(speed, 1);
/// ```
///
/// You can use the `<`, `<=`, `==`, `>=`, `>` with `Speed` values and `i32` values (or other
/// `Speed` values).
///
/// ```rust,no_run
/// # use turtle::{Turtle, Speed};
/// let turtle = Turtle::new();
/// let speed = turtle.speed();
/// if speed == 12 && speed >= 5 && speed < Speed::instant() {
///     println!("Super fast!!");
/// }
/// // This is equivalent, but requires more typing
/// if speed == Speed::from(12) && speed >= Speed::from(5) && speed < Speed::from("instant") {
///     println!("Super fast!!");
/// }
/// ```
///
/// Notice that you can compare `Speed` values to numeric values, but not the other way around.
///
/// ```rust,compile_fail
/// # use turtle::Speed;
/// let speed: Speed = 7.into();
/// if 8 > speed { // This doesn't make sense and won't compile!
///     // ...
/// }
/// ```
///
/// To check if a speed is instant, use the [`is_instant()` method] or compare the speed to
/// [`Speed::instant()`].
///
/// ```rust
/// # use turtle::Speed;
/// let speed = Speed::instant();
/// if speed.is_instant() {
///     println!("Instant!!");
/// }
/// # else { panic!("Speed::instant() was not instant!"); }
/// ```
///
/// [`set_speed` method]: struct.Turtle.html#method.set_speed
/// [`Speed::instant()`]: struct.Speed.html#method.instant
/// [`is_instant()` method]: struct.Speed.html#method.is_instant
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Speed(SpeedLevel);

/// The default speed is "normal"
impl Default for Speed {
    fn default() -> Self {
        "normal".into()
    }
}

impl Speed {
    /// Returns the speed value that will make the turtle move and rotate instantly. This means
    /// that instead of the turtle's movements being animated, it will directly move to wherever
    /// you direct it to go.
    ///
    /// ```rust
    /// # use turtle::{Turtle, Speed};
    /// let mut turtle = Turtle::new();
    /// turtle.set_speed(Speed::instant());
    /// turtle.forward(100.0); // A line will be drawn instantly!
    /// ```
    ///
    /// Whenever possible, you should prefer to use the string `"instant"` instead of calling this
    /// method.
    ///
    /// ```rust
    /// # use turtle::{Turtle};
    /// # let mut turtle = Turtle::new();
    /// turtle.set_speed("instant"); // equivalent to typing `Speed::instant()`
    /// ```
    pub fn instant() -> Self {
        Speed(SpeedLevel::Instant)
    }

    /// Returns true if this speed is the same as `Speed::instant()`
    ///
    /// ```rust
    /// use turtle::{Speed};
    /// let speed: Speed = "instant".into();
    /// assert!(speed.is_instant());
    ///
    /// let speed = Speed::instant();
    /// assert!(speed.is_instant());
    /// ```
    pub fn is_instant(self) -> bool {
        match self {
            Speed(SpeedLevel::Instant) => true,
            _ => false,
        }
    }

    /// Converts a speed to its value as a movement speed in pixels per second
    pub(crate) fn to_px_per_sec(self) -> Distance {
        // Goal: increasing speed causes a linear decrease in the time it takes to draw a line
        //
        // speed = distance / time
        //
        // So we can pick a fixed distance, say 200.0 px, and define a linear set of times that
        // each speed should take.

        let distance = 200.0; // px
        // The time it should take at the minimum speed level (MIN_SPEED) to cross this distance
        let speed_min_time = 2.0 * 1000.0; // ms
        // The time it should take at the maximum speed level (MAX_SPEED) to cross this distance
        let speed_max_time = 5.0; // ms

        use SpeedLevel::*;
        let level = match self.0 {
            Value(speed) => speed,
            // Instant should just be considered as "infinite" speed
            Instant => return f64::INFINITY,
        };

        // Linearly interpolate the time to get the time it should take at this speed level
        // Note:
        //   if level = MIN_SPEED then t = 0.0
        //   if level = MAX_SPEED then t = 1.0
        let t = (level - MIN_SPEED) as f64 / (MAX_SPEED - MIN_SPEED) as f64;
        let time = lerp(&speed_min_time, &speed_max_time, &t); // ms

        // Compute the final speed using the formula above (note: 1000.0 ms == 1.0 s)
        distance * 1000.0 / time
    }

    /// Converts a speed to its value as radians per second
    pub(crate) fn to_rad_per_sec(self) -> Radians {
        // See comment in `to_px_per_sec` for details
        // This is the exact same except we use a value in radians instead of in px for distance

        let distance = radians::TWO_PI; // rad
        let speed_min_time = 2.0 * 1000.0; // ms
        let speed_max_time = 5.0; // ms

        use SpeedLevel::*;
        let level = match self.0 {
            Value(speed) => speed,
            // Instant should just be considered as "infinite" speed
            Instant => return Radians::from_radians_value(f64::INFINITY),
        };

        let t = (level - MIN_SPEED) as f64 / (MAX_SPEED - MIN_SPEED) as f64;
        let time = lerp(&speed_min_time, &speed_max_time, &t); // ms

        distance * 1000.0 / time
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SpeedLevel::*;
        match self.0 {
            Value(value) => fmt::Display::fmt(&value, f),
            Instant => write!(f, "\"instant\""),
        }
    }
}

impl fmt::Debug for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Speed({})", self)
    }
}

impl PartialEq<i32> for Speed {
    fn eq(&self, other: &i32) -> bool {
        self.eq(&Speed::from(*other))
    }
}

impl PartialOrd<i32> for Speed {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.partial_cmp(&Speed::from(*other))
    }
}

impl Random for Speed {
    /// Generates a random speed within the valid range of speed levels
    fn random() -> Self {
        RandomRange::random_range(MIN_SPEED, MAX_SPEED)
    }
}

impl<B: Into<Speed>> RandomRange<B> for Speed {
    /// Generates a random difficulty level within the given range, not including instant.
    ///
    /// # Panics
    ///
    /// Panics if either bound could result in a value outside the valid range of speed levels
    /// or if `low > high`. Also panics if either bound is `Speed::instant()`.
    fn random_range(low: B, high: B) -> Self {
        let low = low.into();
        let high = high.into();
        if let (Speed(SpeedLevel::Value(low)), Speed(SpeedLevel::Value(high))) = (low, high) {
            if low < MIN_SPEED || high > MAX_SPEED {
                panic!("The boundaries must be within the valid range of difficulties");
            }

            Speed(SpeedLevel::Value(RandomRange::random_range(low, high)))
        } else {
            panic!("At least one of the bounds provided to random_range() was Speed::instant()");
        }
    }
}

impl<'a> From<&'a str> for Speed {
    fn from(level_name: &'a str) -> Self {
        use SpeedLevel::*;

        Speed(match level_name {
            "slowest" => Value(1),
            "slower" => Value(5),
            "slow" => Value(8),
            "normal" => Value(10),
            "fast" => Value(12),
            "faster" => Value(15),
            "instant" => Instant,
            _ => panic!(
                "Invalid speed specified, use one of the words: \"slowest\", \"slower\", \"slow\", \"normal\", \"fast\", \"faster\", \"instant\""
            ),
        })
    }
}

impl From<i32> for Speed {
    fn from(n: i32) -> Self {
        use SpeedLevel::*;

        Speed(match n {
            // Special error message for 0 because this used to be a valid speed
            0 => panic!("Invalid speed: 0. If you wanted to set the speed to instant, please use the string \"instant\" or Speed::instant()"),
            n if n >= MIN_SPEED && n <= MAX_SPEED => Value(n),
            n => panic!("Invalid speed: {}. Must be a value between {} and {}", n, MIN_SPEED, MAX_SPEED),
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
    use crate::Turtle;

    #[test]
    fn display() {
        let speed: Speed = "instant".into();
        assert_eq!(format!("{}", speed), "\"instant\"");
        for value in 1..MAX_SPEED {
            let speed: Speed = value.into();
            assert_eq!(format!("{}", speed), format!("{}", value));
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
        assert_eq!(turtle.speed(), Speed::instant());
    }

    #[test]
    #[should_panic(expected = "Invalid speed specified, use one of the words: \"slowest\", \"slower\", \"slow\", \"normal\", \"fast\", \"faster\", \"instant\"")]
    fn invalid_speed() {
        let mut turtle = Turtle::new();
        turtle.set_speed("wrong");
    }

    #[test]
    fn speed_values() {
        let mut turtle = Turtle::new();
        for speed in 1..(MAX_SPEED + 1) {
            turtle.set_speed(speed);
            assert_eq!(turtle.speed(), speed);
        }
    }

    #[test]
    fn speed_values_f64() {
        let mut turtle = Turtle::new();
        for speed in 1..MAX_SPEED {
            turtle.set_speed(speed as f64 + 0.4);
            assert_eq!(turtle.speed(), speed);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid speed: 26. Must be a value between 1 and 25")]
    fn speed_value_out_of_range() {
        let mut turtle = Turtle::new();
        turtle.set_speed(26);
    }

    #[test]
    #[should_panic(expected = "Invalid speed: 20394. Must be a value between 1 and 25")]
    fn speed_value_out_of_range2() {
        let mut turtle = Turtle::new();
        turtle.set_speed(20394);
    }

    #[test]
    #[should_panic(expected = "Invalid speed: -1. Must be a value between 1 and 25")]
    fn speed_value_out_of_range_negative() {
        let mut turtle = Turtle::new();
        turtle.set_speed(-1);
    }

    #[test]
    #[should_panic(expected = "Invalid speed: 0. If you wanted to set the speed to instant, please use the string \"instant\" or Speed::instant()")]
    fn disallow_zero() {
        let mut turtle = Turtle::new();
        turtle.set_speed(0);
    }
}
