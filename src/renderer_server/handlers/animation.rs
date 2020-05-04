use glutin::event_loop::EventLoopProxy;
use tokio::{time, sync::Mutex};
use interpolation::lerp;

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
    RotationDirection,
};
use crate::renderer_client::ClientId;
use crate::radians::Radians;
use crate::{Distance, Point};

use super::super::{
    RequestRedraw,
    state::TurtleState,
    app::{TurtleId, TurtleDrawings},
    access_control::{AccessControl, RequiredData, RequiredTurtles},
    renderer::display_list::{DisplayList, PrimHandle},
};

/// Frames per second - The number of times the animation will update per second
const FPS: u64 = 60;

// 1,000,000 us in 1 s
const MICROS_PER_SEC: u64 = 1_000_000;

pub(crate) async fn move_forward(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: &Mutex<EventLoopProxy<RequestRedraw>>,
    id: TurtleId,
    distance: Distance,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;

    // Borrow the data initially to setup the animation but then drop it so the locks on the turtle
    // and display list are released (allows for rendering to occur)
    let mut anim = {
        let mut turtles = data.turtles_mut().await;
        let turtle = turtles.one_mut();

        let TurtleState {position, heading, ..} = turtle.state;

        // The total amount we'll move in the x and y directions
        let movement = Point {
            x: distance * heading.cos(),
            y: distance * heading.sin(),
        };
        let target_pos = position + movement;

        let mut display_list = display_list.lock().await;
        MoveAnimation::new(turtle, &mut display_list, target_pos)
    };

    while anim.running {
        // Signal the main thread that the image has changed
        event_loop.lock().await.send_event(RequestRedraw)
            .expect("bug: event loop closed before animation completed");

        // Sleep until it is time to update the animation again
        anim.timer.tick().await;

        // These locks are dropped at the end of this loop iteration to allow rendering to occur
        // while we wait
        let mut turtles = data.turtles_mut().await;
        let turtle = turtles.one_mut();
        let mut display_list = display_list.lock().await;

        anim.step(turtle, &mut display_list);
    }

    conn.send(client_id, ServerResponse::AnimationComplete(id)).await
        .expect("unable to send response to IPC client");
}

pub(crate) async fn move_to(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: &Mutex<EventLoopProxy<RequestRedraw>>,
    id: TurtleId,
    target_pos: Point,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;

    // Borrow the data initially to setup the animation but then drop it so the locks on the turtle
    // and display list are released (allows for rendering to occur)
    let mut anim = {
        let mut turtles = data.turtles_mut().await;
        let turtle = turtles.one_mut();

        let mut display_list = display_list.lock().await;
        MoveAnimation::new(turtle, &mut display_list, target_pos)
    };

    while anim.running {
        // Signal the main thread that the image has changed
        event_loop.lock().await.send_event(RequestRedraw)
            .expect("bug: event loop closed before animation completed");

        // Sleep until it is time to update the animation again
        anim.timer.tick().await;

        // These locks are dropped at the end of this loop iteration to allow rendering to occur
        // while we wait
        let mut turtles = data.turtles_mut().await;
        let turtle = turtles.one_mut();
        let mut display_list = display_list.lock().await;

        anim.step(turtle, &mut display_list);
    }

    conn.send(client_id, ServerResponse::AnimationComplete(id)).await
        .expect("unable to send response to IPC client");
}

struct MoveAnimation {
    /// true if the animation should continue, false if it should stop
    running: bool,
    /// A timer used to delay the execution of the animation to a reasonable FPS
    timer: time::Interval,
    /// The instant that the animation started, used to precisely determine how long the animation
    /// has been running
    start: time::Instant,
    /// The position that the turtle started at
    start_pos: Point,
    /// The target position to move to (i.e. the final value of the animation)
    target_pos: Point,
    /// The total duration of the animation in microseconds
    total_micros: f64,
    /// A handle to the line that is manipulated by thsi animation
    prim: PrimHandle,
}

impl MoveAnimation {
    fn new(
        turtle: &mut TurtleDrawings,
        display_list: &mut DisplayList,
        target_pos: Point,
    ) -> Self {
        let frame_duration = time::Duration::from_millis(MICROS_PER_SEC / FPS);
        // Need to start at now() + frame_duration or else timer will initially tick for 0 seconds
        let timer = time::interval_at(time::Instant::now() + frame_duration, frame_duration);

        let TurtleState {position, speed, ref pen, ..} = turtle.state;

        if speed.is_instant() {
            // Set to the final position and draw a line with no animation
            turtle.state.position = target_pos;
            let prim = display_list.push_line(position, target_pos, pen);
            turtle.drawings.push(prim);

            if let Some(poly_handle) = turtle.current_fill_polygon {
                //TODO: Push point into polygon
                todo!()
            }

            Self {
                // stop the animation right away since it has already completed
                running: false,
                timer,
                start: time::Instant::now(),
                start_pos: position,
                target_pos,
                total_micros: 0.0,
                prim,
            }

        } else {
            let px_per_sec = speed.to_px_per_sec();
            let abs_distance = (target_pos - position).len();
            // Use microseconds instead of ms for greater precision
            let total_micros = abs_distance * MICROS_PER_SEC as f64 / px_per_sec;

            // No need to update position since the turtle hasn't move anywhere yet

            // Start with a zero-length line since the animation hasn't started yet
            let prim = display_list.push_line(position, position, pen);
            turtle.drawings.push(prim);

            if let Some(poly_handle) = turtle.current_fill_polygon {
                //TODO: Push point into polygon
                todo!()
            }

            Self {
                running: true,
                timer,
                start: time::Instant::now(),
                start_pos: position,
                target_pos,
                total_micros,
                prim,
            }
        }

    }

    /// Advances the animation based on the amount of time that has elapsed so far
    fn step(&mut self, turtle: &mut TurtleDrawings, display_list: &mut DisplayList) {
        let &mut Self {
            ref mut running,
            timer: _,
            ref mut start,
            start_pos,
            target_pos,
            total_micros,
            prim,
        } = self;

        let elapsed = start.elapsed().as_micros() as f64;
        if elapsed >= total_micros {
            *running = false;
            turtle.state.position = target_pos;
            display_list.replace_line(prim, start_pos, target_pos, &turtle.state.pen);

            if let Some(poly_handle) = turtle.current_fill_polygon {
                //TODO: Replace point in fill polygon
                todo!()
            }

        } else {
            // t is the total progress made in the animation so far
            let t = elapsed / total_micros;
            let current_pos = lerp(&start_pos, &target_pos, &t);

            turtle.state.position = current_pos;
            display_list.replace_line(prim, start_pos, current_pos, &turtle.state.pen);

            if let Some(poly_handle) = turtle.current_fill_polygon {
                //TODO: Replace point in fill polygon
                todo!()
            }
        }
    }
}

pub(crate) async fn rotate_in_place(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    event_loop: &Mutex<EventLoopProxy<RequestRedraw>>,
    id: TurtleId,
    angle: Radians,
    direction: RotationDirection,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, ..} = turtles.one_mut();

    //TODO

    conn.send(client_id, ServerResponse::AnimationComplete(id)).await
        .expect("unable to send response to IPC client");
}
