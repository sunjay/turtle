#[cfg(not(any(feature = "test", test)))]
use interpolation::lerp;

use radians::{self, Radians};
use state::{TurtleState, Path};
use timer::Timer;

pub trait Animation {
    /// Advance the animation forward.
    ///
    /// The animation will use the timer it stores to calculate the current state it should be at.
    fn advance(&self, turtle: &mut TurtleState) -> AnimationStatus;
}

// During tests, we disable animations and so it appears that this is dead code when it is not
// dead code in reality
// See Cargo.toml for an explanation of this attribute
#[cfg_attr(any(feature = "test", test), allow(dead_code))]
pub enum AnimationStatus {
    /// Returned if the animation is still running.
    ///
    /// If a Path is provided, it should be rendered.
    /// Make sure to check whether the path's pen is enabled before rendering.
    Running(Option<Path>),
    /// Returned if the animation is complete.
    /// The advance() function should not be called anymore after this.
    /// The returned path, if any, should be added to the list of paths to be rendered every frame.
    /// Make sure to check whether the path's pen is enabled before rendering.
    Complete(Option<Path>),
}

/// Rotates given angle in the given direction by the given rotation
///
/// Let's say you have a starting angle X. Standard angles go counterclockwise, so
/// if clockwise is true, we need to subtract the `rotation` from X resulting in
/// `X - rotation`. If clockwise is false, we can just add normally.
fn rotate(angle: Radians, rotation: Radians, clockwise: bool) -> Radians {
    let angle = if clockwise { angle - rotation } else { angle + rotation };
    // Normalize the angle to be between 0 and 2*pi
    // Formula adapted from: https://stackoverflow.com/a/24234924/551904
    // More info: https://stackoverflow.com/a/28316446/551904
    angle - radians::TWO_PI * (angle / radians::TWO_PI).floor()
}

/// Animate the turtle moving from a start point to an end point
/// while keeping its heading the same throughout (in a straight line)
pub struct MoveAnimation {
    /// path.start and path.end are used in lerp
    pub path: Path,
    /// timer since the start of the animation
    ///
    /// used with total_millis to calculate t in lerp
    pub timer: Timer,
    /// the total time the animation is meant to take based on the speed and length from
    /// start to finish
    ///
    /// if this is zero, we'll jump to the end
    pub total_millis: f64,
}

impl Animation for MoveAnimation {
    /// Advance the animation forward.
    ///
    /// The animation will use the timer it stores to calculate the current state it should be at.
    // See Cargo.toml for an explanation of this attribute
    #[cfg(not(any(feature = "test", test)))]
    fn advance(&self, turtle: &mut TurtleState) -> AnimationStatus {
        use self::AnimationStatus::*;

        let MoveAnimation {ref path, ref timer, total_millis} = *self;
        let elapsed = timer.elapsed_millis() as f64;
        if elapsed >= total_millis {
            turtle.position = path.end;
            Complete(Some(path.clone()))
        }
        else {
            // t is the total progress made in the animation so far
            let t = elapsed / total_millis;
            turtle.position = lerp(&path.start, &path.end, &t);

            Running(Some(Path {
                start: path.start,
                end: turtle.position,
                pen: path.pen.clone(),
            }))
        }
    }

    // See Cargo.toml for an explanation of this attribute
    #[cfg(any(feature = "test", test))]
    fn advance(&self, turtle: &mut TurtleState) -> AnimationStatus {
        use self::AnimationStatus::*;

        // No animation during testing
        let MoveAnimation {ref path, ..} = *self;
        turtle.position = path.end;
        Complete(Some(path.clone()))
    }
}

/// Rotate the turtle in place by the given angle in the direction specified
pub struct RotateAnimation {
    /// turtle.heading when the animation was started
    pub start: Radians,
    /// The total angle to move by.
    ///
    /// Linearly interpolated based on the elapsed time to get the delta that should be added
    /// to start to get the current heading in the animation
    pub delta_angle: Radians,
    /// The direction of rotation
    pub clockwise: bool,
    /// timer since the start of the animation
    ///
    /// used with total_millis to calculate t in lerp
    pub timer: Timer,
    /// the total time the animation is meant to take based on the speed and length from
    /// start to finish
    ///
    /// if this is zero, we'll jump to the end
    pub total_millis: f64,
}

impl Animation for RotateAnimation {
    /// Advance the animation forward.
    ///
    /// The animation will use the timer it stores to calculate the current state it should be at.
    // See Cargo.toml for an explanation of this attribute
    #[cfg(not(any(feature = "test", test)))]
    fn advance(&self, turtle: &mut TurtleState) -> AnimationStatus {
        use self::AnimationStatus::*;

        let RotateAnimation {start, delta_angle, clockwise, ref timer, total_millis} = *self;
        let elapsed = timer.elapsed_millis() as f64;

        if elapsed >= total_millis {
            turtle.heading = rotate(start, delta_angle, clockwise);

            Complete(None)
        }
        else {
            // t is the total progress made in the animation so far
            let t = elapsed / total_millis;
            // Only rotate as much as the animation has proceeded so far
            let angle = lerp(&radians::ZERO, &delta_angle, &t);
            turtle.heading = rotate(start, angle, clockwise);
            assert!(!turtle.heading.is_nan(), "bug: heading became NaN");

            Running(None)
        }
    }

    // See Cargo.toml for an explanation of this attribute
    #[cfg(any(feature = "test", test))]
    fn advance(&self, turtle: &mut TurtleState) -> AnimationStatus {
        use self::AnimationStatus::*;

        // No animation during testing
        let RotateAnimation {start, delta_angle, clockwise, ..} = *self;
        turtle.heading = rotate(start, delta_angle, clockwise);

        Complete(None)
    }
}
