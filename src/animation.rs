use std::time::Instant;

use interpolation::lerp;

use radians::{self, Radians, TWO_PI};
use drawing_thread::{TurtleState, Path};
use extensions::AsMillis;

pub enum Animation {
    /// Animate the turtle moving from a start point to an end point
    /// while keeping its heading the same throughout (in a straight line)
    Move {
        /// path.start and path.end are used in lerp
        path: Path,
        ‎/// timer since the start of the animation
        ///
        ‎/// used with total_millis to calculate t in lerp
        ‎timer: Instant,
        ‎/// the total time the animation is meant to take based on the speed and length from
        /// start to finish
        ///
        ‎/// if this is zero, we'll jump to the end
        ‎total_millis: f64,
    },

    /// Rotate the turtle in place by the given angle in the direction specified
    Rotate {
        /// turtle.heading when the animation was started
        start: Radians,
        ‎/// The total angle to move by.
        ///
        ‎/// Linearly interpolated based on the elapsed time to get the delta that should be added
        /// to start to get the current heading in the animation
        ‎delta_angle: Radians,
        /// The direction of rotation
        ‎clockwise: bool,
        ‎/// timer since the start of the animation
        ///
        ‎/// used with total_millis to calculate t in lerp
        ‎timer: Instant,
        ‎/// the total time the animation is meant to take based on the speed and length from
        /// start to finish
        ///
        ‎/// if this is zero, we'll jump to the end
        ‎total_millis: f64,
    },
}

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
    if clockwise {
        angle - rotation
    }
    else {
        angle + rotation
    }
}

impl Animation {
    /// Advance the animation forward.
    ///
    /// The animation will use the timer it stores to calculate the current state it should be at.
    pub fn advance(&self, turtle: &mut TurtleState) -> AnimationStatus {
        use self::Animation::*;
        use self::AnimationStatus::*;

        match *self {
            Move {ref path, timer, total_millis} => {
                let elapsed = timer.elapsed().as_millis() as f64;
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
            },
            Rotate {start, delta_angle, clockwise, timer, total_millis} => {
                let elapsed = timer.elapsed().as_millis() as f64;

                if elapsed >= total_millis {
                    turtle.heading = rotate(start, delta_angle, clockwise) % TWO_PI;

                    Complete(None)
                }
                else {
                    // t is the total progress made in the animation so far
                    let t = elapsed / total_millis;
                    // Only rotate as much as the animation has proceeded so far
                    let angle = lerp(&radians::ZERO, &delta_angle, &t);
                    turtle.heading = rotate(start, angle, clockwise) % TWO_PI;
                    assert!(!turtle.heading.is_nan(), "bug: heading became NaN");

                    Running(None)
                }
            }
        }
    }
}
