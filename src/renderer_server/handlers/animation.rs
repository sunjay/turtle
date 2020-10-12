use crate::ipc_protocol::{RotationDirection, ServerOneshotSender, ServerResponse};
use crate::radians::Radians;
use crate::{Distance, Point};

use super::super::{
    animation::{AnimationRunner, MoveAnimation, RotateAnimation},
    app::{App, TurtleId},
    event_loop_notifier::EventLoopNotifier,
    renderer::display_list::DisplayList,
    state::TurtleState,
};
use super::HandlerError;

pub(crate) fn move_forward(
    conn: ServerOneshotSender,
    app: &mut App,
    display_list: &mut DisplayList,
    event_loop: &EventLoopNotifier,
    anim_runner: &AnimationRunner,
    id: TurtleId,
    distance: Distance,
) -> Result<(), HandlerError> {
    let turtle = app.turtle_mut(id);

    let TurtleState {
        position, heading, ..
    } = turtle.state;

    // The total amount we'll move in the x and y directions
    let movement = Point {
        x: distance * heading.cos(),
        y: distance * heading.sin(),
    };
    let target_pos = position + movement;

    let anim = MoveAnimation::new(turtle, display_list, target_pos);

    if anim.is_running() {
        anim_runner.play(id, anim, conn.client_id());
    } else {
        // Instant animations complete right away and don't need to be queued
        // Signal the main thread that the image has changed
        event_loop.request_redraw()?;

        conn.send(ServerResponse::AnimationComplete(id))?;
    }

    Ok(())
}

pub(crate) fn move_to(
    conn: ServerOneshotSender,
    app: &mut App,
    display_list: &mut DisplayList,
    event_loop: &EventLoopNotifier,
    anim_runner: &AnimationRunner,
    id: TurtleId,
    target_pos: Point,
) -> Result<(), HandlerError> {
    let turtle = app.turtle_mut(id);

    let anim = MoveAnimation::new(turtle, display_list, target_pos);

    if anim.is_running() {
        anim_runner.play(id, anim, conn.client_id());
    } else {
        // Instant animations complete right away and don't need to be queued
        // Signal the main thread that the image has changed
        event_loop.request_redraw()?;

        conn.send(ServerResponse::AnimationComplete(id))?;
    }

    Ok(())
}

pub(crate) fn rotate_in_place(
    conn: ServerOneshotSender,
    app: &mut App,
    event_loop: &EventLoopNotifier,
    anim_runner: &AnimationRunner,
    id: TurtleId,
    angle: Radians,
    direction: RotationDirection,
) -> Result<(), HandlerError> {
    let turtle = app.turtle_mut(id);

    let anim = RotateAnimation::new(turtle, angle, direction);

    if anim.is_running() {
        anim_runner.play(id, anim, conn.client_id());
    } else {
        // Instant animations complete right away and don't need to be queued
        // Signal the main thread that the image has changed
        event_loop.request_redraw()?;

        conn.send(ServerResponse::AnimationComplete(id))?;
    }

    Ok(())
}
