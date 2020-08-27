use std::cmp::min;

use tokio::time;
use interpolation::lerp;

use crate::renderer_client::ClientId;
use crate::ipc_protocol::RotationDirection;
use crate::radians::{self, Radians};
use crate::Point;

use super::super::{
    state::TurtleState,
    renderer::display_list::{DisplayList, PrimHandle},
};

use super::TurtleDrawings;

/// Frames per second - The number of times the animation will update per second
const FPS: u64 = 60;

/// 1,000,000 us in 1 s
const MICROS_PER_SEC: u64 = 1_000_000;

/// The maximum length of an animation frame
const FRAME_DURATION: time::Duration = time::Duration::from_micros(MICROS_PER_SEC / FPS);

/// Each animation and client that will be notified when that animation is completed
#[derive(Debug)]
pub enum Animation {
    Move(MoveAnimation, ClientId),
    Rotate(RotateAnimation, ClientId),
}

#[derive(Debug)]
pub struct MoveAnimation {
    /// true if the animation should continue, false if it should stop
    running: bool,
    /// The next amount of time the animation loop should be delayed, up to `FRAME_DURATION`
    ///
    /// Updated with every call to `step`
    next_delay: time::Duration,
    /// The instant that the animation started, used to precisely determine how long the animation
    /// has been running
    start: time::Instant,
    /// The position that the turtle started at
    start_pos: Point,
    /// The target position to move to (i.e. the final value of the animation)
    target_pos: Point,
    /// The total duration of the animation
    total_duration: time::Duration,
    /// A handle to the line that is manipulated by this animation (if any)
    prim: Option<PrimHandle>,
    /// The index of this point in the fill polygon (if any)
    fill_poly_index: Option<usize>,
}

impl MoveAnimation {
    pub fn new(
        turtle: &mut TurtleDrawings,
        display_list: &mut DisplayList,
        target_pos: Point,
    ) -> Self {
        let TurtleState {position, speed, ref pen, ..} = turtle.state;

        if cfg!(any(feature = "test", test)) || speed.is_instant() {
            // Set to the final position and draw a line with no animation
            turtle.state.position = target_pos;
            let prim = display_list.push_line(position, target_pos, pen);
            turtle.drawings.extend(prim);

            // Append to the current fill polygon, if any
            let fill_poly_index = turtle.current_fill_polygon.map(|poly_handle| {
                display_list.polygon_push(poly_handle, position)
            });

            Self {
                // stop the animation right away since it has already completed
                running: false,
                next_delay: time::Duration::from_micros(0),
                start: time::Instant::now(),
                start_pos: position,
                target_pos,
                total_duration: time::Duration::from_micros(0),
                prim,
                fill_poly_index,
            }

        } else {
            let px_per_sec = speed.to_px_per_sec();
            let abs_distance = (target_pos - position).len();
            // Use microseconds instead of ms for greater precision
            let total_micros = abs_distance * MICROS_PER_SEC as f64 / px_per_sec;
            let total_duration = time::Duration::from_micros(total_micros as u64);

            // If the duration of the animation is less than a frame, don't wait the entire frame
            // to complete it
            let next_delay = min(total_duration, FRAME_DURATION);

            // No need to update position since the turtle hasn't move anywhere yet

            // Start with a zero-length line since the animation hasn't started yet
            let prim = display_list.push_line(position, position, pen);
            turtle.drawings.extend(prim);

            // Append to the current fill polygon, if any
            let fill_poly_index = turtle.current_fill_polygon.map(|poly_handle| {
                display_list.polygon_push(poly_handle, position)
            });

            Self {
                running: true,
                next_delay,
                start: time::Instant::now(),
                start_pos: position,
                target_pos,
                total_duration,
                prim,
                fill_poly_index,
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Advances the animation based on the amount of time that has elapsed so far
    pub fn step(&mut self, turtle: &mut TurtleDrawings, display_list: &mut DisplayList) {
        let &mut Self {
            ref mut running,
            ref mut next_delay,
            ref start,
            start_pos,
            target_pos,
            total_duration,
            prim,
            fill_poly_index,
        } = self;

        let elapsed = start.elapsed();
        let pos = if elapsed >= total_duration {
            *running = false;
            *next_delay = time::Duration::from_micros(0);

            target_pos

        } else {
            // t is the total progress made in the animation so far
            let t = elapsed.as_micros() as f64 / total_duration.as_micros() as f64;
            let current_pos = lerp(&start_pos, &target_pos, &t);

            // If the time remaining is less than a frame, don't wait the entire frame
            let remaining = total_duration - elapsed;
            *next_delay = min(remaining, FRAME_DURATION);

            current_pos
        };

        // Update state with the current position
        turtle.state.position = pos;

        // Update the end of the line we have been drawing, if any
        if let Some(prim) = prim {
            display_list.line_update_end(prim, pos);
        }

        // Replace the point in the current fill polygon, if any
        if let Some(poly_handle) = turtle.current_fill_polygon {
            // This unwrap is safe because `current_fill_polygon` is `Some`
            display_list.polygon_update(poly_handle, fill_poly_index.unwrap(), pos);
        }
    }
}

#[derive(Debug)]
pub struct RotateAnimation {
    /// true if the animation should continue, false if it should stop
    running: bool,
    /// The next amount of time the animation loop should be delayed, up to `FRAME_DURATION`
    ///
    /// Updated with every call to `step`
    next_delay: time::Duration,
    /// The instant that the animation started, used to precisely determine how long the animation
    /// has been running
    start: time::Instant,
    /// The start angle of the turtle
    start_heading: Radians,
    /// The angle in radians that the turtle is to have rotated by the end of the animation
    delta_angle: Radians,
    /// The direction of rotation
    direction: RotationDirection,
    /// The total duration of the animation
    total_duration: time::Duration,
}

impl RotateAnimation {
    pub fn new(
        turtle: &mut TurtleDrawings,
        delta_angle: Radians,
        direction: RotationDirection,
    ) -> Self {
        let TurtleState {heading, speed, ..} = turtle.state;
        if cfg!(any(feature = "test", test)) || speed.is_instant() {
            // Set to the final heading with no animation
            turtle.state.heading = rotate(heading, delta_angle, direction);

            Self {
                // stop the animation right away since it has already completed
                running: false,
                next_delay: time::Duration::from_micros(0),
                start: time::Instant::now(),
                start_heading: heading,
                delta_angle,
                direction,
                total_duration: time::Duration::from_micros(0),
            }

        } else {
            let rad_per_sec = speed.to_rad_per_sec();
            // Use microseconds instead of ms for greater precision
            let total_micros = (delta_angle * MICROS_PER_SEC as f64 / rad_per_sec).to_radians();
            // abs() because time is always positive, even if angle is negative
            let total_duration = time::Duration::from_micros(total_micros.abs() as u64);

            // If the duration of the animation is less than a frame, don't wait the entire frame
            // to complete it
            let next_delay = min(total_duration, FRAME_DURATION);

            // No need to update heading since the turtle hasn't rotated yet

            Self {
                running: true,
                next_delay,
                start: time::Instant::now(),
                start_heading: heading,
                delta_angle,
                direction,
                total_duration,
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Advances the animation based on the amount of time that has elapsed so far
    pub fn step(&mut self, turtle: &mut TurtleDrawings) {
        let &mut Self {
            ref mut running,
            ref mut next_delay,
            ref start,
            start_heading,
            delta_angle,
            direction,
            total_duration,
        } = self;

        let elapsed = start.elapsed();
        let heading = if elapsed >= total_duration {
            *running = false;
            *next_delay = time::Duration::from_micros(0);

            // Set to the final heading
            rotate(start_heading, delta_angle, direction)

        } else {
            // t is the total progress made in the animation so far
            let t = elapsed.as_micros() as f64 / total_duration.as_micros() as f64;
            let current_delta = lerp(&radians::ZERO, &delta_angle, &t);

            // If the time remaining is less than a frame, don't wait the entire frame
            let remaining = total_duration - elapsed;
            *next_delay = min(remaining, FRAME_DURATION);

            rotate(start_heading, current_delta, direction)
        };

        turtle.state.heading = heading;
        debug_assert!(!heading.is_nan(), "bug: heading became NaN");
    }
}

/// Rotates the given `angle` by the given `rotation` in the given `direction`
///
/// Let's say you have a starting angle X. Standard angles go counterclockwise, so
/// if clockwise is true, we need to subtract the `rotation` from X resulting in
/// `X - rotation`. If clockwise is false, we can just add normally.
fn rotate(angle: Radians, rotation: Radians, direction: RotationDirection) -> Radians {
    use RotationDirection::*;
    let angle = match direction {
        Clockwise => angle - rotation,
        Counterclockwise => angle + rotation,
    };

    // Normalize the angle to be between 0 and 2*pi
    // Formula adapted from: https://stackoverflow.com/a/24234924/551904
    // More info: https://stackoverflow.com/a/28316446/551904
    angle - radians::TWO_PI * (angle / radians::TWO_PI).floor()
}
