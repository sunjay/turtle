use std::sync::Arc;
use std::cmp::min;
use std::collections::HashMap;

use tokio::{sync::Notify, time};
use interpolation::lerp;

use crate::renderer_client::ClientId;
use crate::ipc_protocol::{ServerSender, RotationDirection, ServerResponse};
use crate::radians::{self, Radians};
use crate::Point;

use super::super::{
    handle_handler_result,
    state::TurtleState,
    renderer::display_list::{DisplayList, SharedDisplayList, PrimHandle},
    event_loop_notifier::EventLoopNotifier,
    handlers::HandlerError,
};

use super::{SharedApp, App, TurtleDrawings, TurtleId};

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

impl Animation {
    pub fn is_running(&self) -> bool {
        use Animation::*;
        match self {
            Move(anim, _) => anim.is_running(),
            Rotate(anim, _) => anim.is_running(),
        }
    }

    pub fn next_update(&self) -> time::Instant {
        use Animation::*;
        match self {
            Move(anim, _) => anim.next_update(),
            Rotate(anim, _) => anim.next_update(),
        }
    }

    pub fn client_id(&self) -> ClientId {
        use Animation::*;
        match self {
            &Move(_, id) |
            &Rotate(_, id) => id,
        }
    }
}

#[derive(Debug)]
pub struct MoveAnimation {
    /// true if the animation should continue, false if it should stop
    running: bool,
    /// The next instant at which the animation loop should step this animation, up to
    /// `FRAME_DURATION` from now
    ///
    /// Updated with every call to `step`
    next_update: time::Instant,
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

        let start = time::Instant::now();

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
                next_update: start,
                start,
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
            let next_update = start + next_delay;

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
                next_update,
                start,
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

    pub fn next_update(&self) -> time::Instant {
        self.next_update
    }

    /// Advances the animation based on the amount of time that has elapsed since it started
    pub fn step(
        &mut self,
        now: time::Instant,
        state: &mut TurtleState,
        current_fill_polygon: Option<PrimHandle>,
        display_list: &mut DisplayList,
    ) {
        let &mut Self {
            ref mut running,
            ref mut next_update,
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
            *next_update = now;

            target_pos

        } else {
            // t is the total progress made in the animation so far
            let t = elapsed.as_micros() as f64 / total_duration.as_micros() as f64;
            let current_pos = lerp(&start_pos, &target_pos, &t);

            // If the time remaining is less than a frame, don't wait the entire frame
            let remaining = total_duration - elapsed;
            *next_update = now + min(remaining, FRAME_DURATION);

            current_pos
        };

        // Update state with the current position
        state.position = pos;

        // Update the end of the line we have been drawing, if any
        if let Some(prim) = prim {
            display_list.line_update_end(prim, pos);
        }

        // Replace the point in the current fill polygon, if any
        if let Some(poly_handle) = current_fill_polygon {
            // This unwrap is safe because `current_fill_polygon` is `Some`
            display_list.polygon_update(poly_handle, fill_poly_index.unwrap(), pos);
        }
    }
}

#[derive(Debug)]
pub struct RotateAnimation {
    /// true if the animation should continue, false if it should stop
    running: bool,
    /// The next instant at which the animation loop should step this animation, up to
    /// `FRAME_DURATION` from now
    ///
    /// Updated with every call to `step`
    next_update: time::Instant,
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

        let start = time::Instant::now();

        if cfg!(any(feature = "test", test)) || speed.is_instant() {
            // Set to the final heading with no animation
            turtle.state.heading = rotate(heading, delta_angle, direction);

            Self {
                // stop the animation right away since it has already completed
                running: false,
                next_update: start,
                start,
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
            let next_update = start + next_delay;

            // No need to update heading since the turtle hasn't rotated yet

            Self {
                running: true,
                next_update,
                start,
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

    pub fn next_update(&self) -> time::Instant {
        self.next_update
    }

    /// Advances the animation based on the amount of time that has elapsed since it started
    pub fn step(&mut self, now: time::Instant, state: &mut TurtleState) {
        let &mut Self {
            ref mut running,
            ref mut next_update,
            ref start,
            start_heading,
            delta_angle,
            direction,
            total_duration,
        } = self;

        let elapsed = start.elapsed();
        let heading = if elapsed >= total_duration {
            *running = false;
            *next_update = now;

            // Set to the final heading
            rotate(start_heading, delta_angle, direction)

        } else {
            // t is the total progress made in the animation so far
            let t = elapsed.as_micros() as f64 / total_duration.as_micros() as f64;
            let current_delta = lerp(&radians::ZERO, &delta_angle, &t);

            // If the time remaining is less than a frame, don't wait the entire frame
            let remaining = total_duration - elapsed;
            *next_update = now + min(remaining, FRAME_DURATION);

            rotate(start_heading, current_delta, direction)
        };

        state.heading = heading;
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

/// Spawns a task to manage running animations and drive them to completion
#[derive(Debug)]
pub struct AnimationRunner {
    /// Notifies the animation loop that a new animation has been added so it needs to update its
    /// timing info
    new_animation_notifier: Arc<Notify>,
}

impl AnimationRunner {
    pub fn new(
        conn: ServerSender,
        app: SharedApp,
        display_list: SharedDisplayList,
        event_loop: EventLoopNotifier,
    ) -> Self {
        let new_animation_notifier = Arc::new(Notify::new());

        tokio::spawn(animation_loop(
            conn,
            app,
            display_list,
            event_loop,
            new_animation_notifier.clone(),
        ));

        Self {new_animation_notifier}
    }

    /// Notify the animation runner that a new animation has been added
    ///
    /// If this is not called when the animation field is set, it may result in animations not
    /// completing on time. Specifically, this can occur when the animation starts with less than a
    /// frame duration remaining.
    pub fn notify_animation_added(&self) {
        self.new_animation_notifier.notify();
    }
}

/// Drives the animations to completion by updating animations
async fn animation_loop(
    conn: ServerSender,
    app: SharedApp,
    display_list: SharedDisplayList,
    event_loop: EventLoopNotifier,
    new_animation_notifier: Arc<Notify>,
) {
    // Map of turtle ID to the next instant at which the turtle's current animation needs to updated
    let mut anim_next_update = HashMap::new();

    loop {
        let next_update = handle_handler_result(update_animations(
            &conn,
            &mut app.write(),
            &mut display_list.lock(),
            &event_loop,
            &mut anim_next_update,
        ));

        let next_update = match next_update {
            Some(next_update) => next_update,
            None => break,
        };

        tokio::select! {
            _ = time::delay_until(next_update) => {},

            // If a new animation is added, we need to update the current animations and potentially
            // compute a new next delay
            _ = new_animation_notifier.notified() => {},
        }
    }
}

/// Updates all the animations, returning the minimum instant of time to wait until before updating
/// again
///
/// Only the animations whose instant in `anim_next_update` has elapsed will be updated. If an
/// animation is not currently stored there, it will be added.
fn update_animations(
    conn: &ServerSender,
    app: &mut App,
    display_list: &mut DisplayList,
    event_loop: &EventLoopNotifier,
    anim_next_update: &mut HashMap<TurtleId, time::Instant>,
) -> Result<time::Instant, HandlerError> {
    // true if even one animation was updated
    let mut animation_updated = false;

    // It's important to update as soon as an animation is ready to be updated because otherwise we
    // may delay the sending of the AnimationComplete message. Without that, if a turtle is drawing
    // many small lines that take less than a frame duration, it may have to wait too long in
    // between lines. That would make Speed stop mattering under a certain line length and would
    // impose an undesirable minimum amount of time on each animation.
    let now = time::Instant::now();
    let mut min_next_update = now + FRAME_DURATION;

    for (id, turtle) in app.turtles_mut() {
        let TurtleDrawings {state, drawings: _, current_fill_polygon, animation} = turtle;

        if let Some(anim) = animation {
            // Don't update more than necessary
            match anim_next_update.get(&id) {
                Some(&next_update) => {
                    if now < next_update {
                        continue;
                    }
                },

                None => {
                    // Insert the next update time
                    let next_update = anim.next_update();
                    anim_next_update.insert(id, next_update);
                    if now < next_update {
                        continue;
                    }
                }
            }

            use Animation::*;
            match anim {
                Move(anim, _) => anim.step(now, state, *current_fill_polygon, display_list),
                Rotate(anim, _) => anim.step(now, state),
            }

            // Check if the animation was completed
            if anim.is_running() {
                // Wait for the amount of time remaining in this animation, up to the length of an
                // animation frame
                let next_update = anim.next_update();
                min_next_update = min(min_next_update, next_update);

                anim_next_update.insert(id, next_update);

            } else {
                conn.send(anim.client_id(), ServerResponse::AnimationComplete(id))?;

                *animation = None;

                anim_next_update.remove(&id);
            }

            animation_updated = true;
        }
    }

    if animation_updated {
        event_loop.request_redraw()?;
    }

    Ok(min_next_update)
}
