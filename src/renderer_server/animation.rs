use std::cmp::min;
use std::collections::HashMap;

use tokio::{sync::mpsc, time};
use interpolation::lerp;
use parking_lot::{RwLock, Mutex};

use crate::renderer_client::ClientId;
use crate::ipc_protocol::{ServerSender, RotationDirection, ServerResponse};
use crate::radians::{self, Radians};
use crate::Point;

use super::{
    handle_handler_result,
    app::{SharedApp, App, TurtleDrawings, TurtleId},
    state::TurtleState,
    renderer::display_list::{DisplayList, SharedDisplayList, PrimHandle},
    event_loop_notifier::EventLoopNotifier,
    handlers::HandlerError,
};

/// Frames per second - The number of times the animation will update per second
const FPS: u64 = 60;

/// 1,000,000 us in 1 s
const MICROS_PER_SEC: u64 = 1_000_000;

/// The maximum length of an animation frame
const FRAME_DURATION: time::Duration = time::Duration::from_micros(MICROS_PER_SEC / FPS);

#[derive(Debug)]
pub enum AnimationKind {
    Move(MoveAnimation),
    Rotate(RotateAnimation),
}

impl From<MoveAnimation> for AnimationKind {
    fn from(anim: MoveAnimation) -> Self {
        AnimationKind::Move(anim)
    }
}

impl From<RotateAnimation> for AnimationKind {
    fn from(anim: RotateAnimation) -> Self {
        AnimationKind::Rotate(anim)
    }
}

#[derive(Debug)]
struct Animation {
    /// The ID of the turtle associated with this animation
    turtle_id: TurtleId,
    /// The animation that will be played
    kind: AnimationKind,
    /// The client that will be notified when that animation is completed
    client_id: ClientId,
}

impl Animation {
    pub fn new(turtle_id: TurtleId, kind: impl Into<AnimationKind>, client_id: ClientId) -> Self {
        let kind = kind.into();
        Self {turtle_id, kind, client_id}
    }

    pub fn is_running(&self) -> bool {
        use AnimationKind::*;
        match &self.kind {
            Move(anim) => anim.is_running(),
            Rotate(anim) => anim.is_running(),
        }
    }

    pub fn next_update(&self) -> time::Instant {
        use AnimationKind::*;
        match &self.kind {
            Move(anim) => anim.next_update(),
            Rotate(anim) => anim.next_update(),
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
    /// The current position of the turtle (updated by step)
    current_pos: Point,
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
                current_pos: position,
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
                current_pos: position,
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
    pub fn step(&mut self, now: time::Instant) {
        let &mut Self {
            ref mut running,
            ref mut next_update,
            ref start,
            start_pos,
            target_pos,
            ref mut current_pos,
            total_duration,
            prim: _,
            fill_poly_index: _,
        } = self;

        let elapsed = start.elapsed();
        *current_pos = if elapsed >= total_duration {
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
    }

    pub fn write_current_state(
        &self,
        state: &mut TurtleState,
        current_fill_polygon: Option<PrimHandle>,
        display_list: &mut DisplayList,
    ) {
        let pos = self.current_pos;

        // Update state with the current position
        state.position = pos;

        // Update the end of the line we have been drawing, if any
        if let Some(prim) = self.prim {
            display_list.line_update_end(prim, pos);
        }

        // Replace the point in the current fill polygon, if any
        if let Some(poly_handle) = current_fill_polygon {
            // This unwrap is safe because `current_fill_polygon` is `Some`
            display_list.polygon_update(poly_handle, self.fill_poly_index.unwrap(), pos);
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
    /// The current angle of the turtle (updated by step)
    current_heading: Radians,
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
                current_heading: heading,
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
                current_heading: heading,
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
    pub fn step(&mut self, now: time::Instant) {
        let &mut Self {
            ref mut running,
            ref mut next_update,
            ref start,
            start_heading,
            ref mut current_heading,
            delta_angle,
            direction,
            total_duration,
        } = self;

        let elapsed = start.elapsed();
        *current_heading = if elapsed >= total_duration {
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
    }

    pub fn write_current_state(&self, state: &mut TurtleState) {
        state.heading = self.current_heading;
        debug_assert!(!self.current_heading.is_nan(), "bug: heading became NaN");
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

#[derive(Debug)]
enum Message {
    /// Run the given animation
    Play(Animation),
    /// Stop all animations that are currently playing
    ///
    /// Animations stop at wherever they were last updated.
    StopAll,
}

/// Spawns a task to manage running animations and drive them to completion
#[derive(Debug)]
pub struct AnimationRunner {
    sender: mpsc::UnboundedSender<Message>,
}

impl AnimationRunner {
    pub fn new(
        conn: ServerSender,
        app: SharedApp,
        display_list: SharedDisplayList,
        event_loop: EventLoopNotifier,
    ) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(animation_loop(
            conn,
            app,
            display_list,
            event_loop,
            receiver,
        ));

        Self {sender}
    }

    pub fn play(&self, turtle_id: TurtleId, kind: impl Into<AnimationKind>, client_id: ClientId) {
        self.send(Message::Play(Animation::new(turtle_id, kind, client_id)));
    }

    pub fn stop_all(&self) {
        self.send(Message::StopAll);
    }

    fn send(&self, mess: Message) {
        self.sender.send(mess)
            .expect("bug: animation runner task should run as long as server task");
    }
}

/// Drives the animations to completion by updating animations
async fn animation_loop(
    conn: ServerSender,
    app: SharedApp,
    display_list: SharedDisplayList,
    event_loop: EventLoopNotifier,
    mut receiver: mpsc::UnboundedReceiver<Message>,
) {
    // Map of turtle ID to the current animation playing for it (if any)
    let mut animations: HashMap<TurtleId, Animation> = HashMap::new();

    let mut next_frame = time::Instant::now() + FRAME_DURATION;
    // It's important to update as soon as an animation is ready to be updated because otherwise we
    // may delay the sending of the AnimationComplete message. Without that, if a turtle is drawing
    // many small lines that take less than a frame duration, it may have to wait too long in
    // between lines. That would make Speed stop mattering under a certain line length and would
    // impose an undesirable minimum amount of time on each animation.
    let mut next_update = compute_next_update(next_frame, &animations);

    loop {
        tokio::select! {
            mess = receiver.recv() => match mess {
                Some(Message::Play(anim)) => {
                    // Insert the new animation so we can account for it when selecting the next
                    // update time. Keeping the previous next frame value since we don't want to
                    // bump to another future frame just because we got another animation.
                    debug_assert!(!animations.contains_key(&anim.turtle_id),
                        "bug: cannot animate turtle while another animation is playing");
                    animations.insert(anim.turtle_id, anim);
                },

                Some(Message::StopAll) => {
                    // Complete all pending animations at their last update
                    for anim in animations.values() {
                        handle_handler_result(conn.send(
                            anim.client_id,
                            ServerResponse::AnimationComplete(anim.turtle_id),
                        ).map_err(HandlerError::IpcChannelError));
                    }

                    animations.clear();
                },

                // Sender has been dropped, so renderer server has stopped running
                None => break,
            },

            // Trigger an update once the next update time has elapsed
            _ = time::delay_until(next_update) => {
                let now = time::Instant::now();

                handle_handler_result(update_animations(
                    now,
                    &conn,
                    &app,
                    &display_list,
                    &event_loop,
                    &mut animations,
                ));

                // Only advance if the frame has elapsed
                //
                // This loop should only ever execute once, but we're still using a loop here to
                // guarantee that we never fall behind
                while now >= next_frame {
                    // Advance the next frame based on its previous value, not based on `now` since
                    // that may be any arbitrary time.
                    next_frame += FRAME_DURATION;
                }
            },
        }

        // Set the time at which we should schedule the next update
        next_update = compute_next_update(next_frame, &animations);
    }
}

/// Compute the time of the next update, returning a value up to the time of the next frame
fn compute_next_update(
    next_frame: time::Instant,
    animations: &HashMap<TurtleId, Animation>,
) -> time::Instant {
    let next_update = animations.values()
        .map(|anim| anim.next_update())
        .min()
        .unwrap_or(next_frame);

    min(next_update, next_frame)
}

/// Updates all animations that are ready to be updated again based on the last time they were
/// updated
fn update_animations(
    now: time::Instant,
    conn: &ServerSender,
    app: &RwLock<App>,
    display_list: &Mutex<DisplayList>,
    event_loop: &EventLoopNotifier,
    animations: &mut HashMap<TurtleId, Animation>,
) -> Result<(), HandlerError> {
    // true if even one animation was updated
    let mut animation_updated = false;

    let mut completed_animations = Vec::new();
    for anim in animations.values_mut() {
        // Only update animations when they are ready to be updated
        if now < anim.next_update() {
            continue;
        }

        use AnimationKind::*;
        match &mut anim.kind {
            Move(anim) => anim.step(now),
            Rotate(anim) => anim.step(now),
        }

        // Check if the animation has completed
        if !anim.is_running() {
            conn.send(anim.client_id, ServerResponse::AnimationComplete(anim.turtle_id))?;

            completed_animations.push(anim.turtle_id);
        }

        animation_updated = true;
    }

    if animation_updated {
        // Update turtles after animations have been updated to keep the critical section as small
        // as possible
        let mut app = app.write();
        let mut display_list = display_list.lock();
        for anim in animations.values_mut() {
            let TurtleDrawings {state, current_fill_polygon, ..} = app.turtle_mut(anim.turtle_id);

            use AnimationKind::*;
            match &anim.kind {
                Move(anim) => {
                    anim.write_current_state(state, *current_fill_polygon, &mut display_list);
                },

                Rotate(anim) => {
                    anim.write_current_state(state);
                },
            }
        }

        // Only request a redraw if an update occurred
        event_loop.request_redraw()?;
    }

    // Wait to remove the completed animations so we have a chance to update the turtles with the
    // final state of each animation
    for id in completed_animations {
        animations.remove(&id);
    }

    Ok(())
}
