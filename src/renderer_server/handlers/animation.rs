use tokio::sync::Mutex;

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
    RotationDirection,
};
use crate::renderer_client::ClientId;
use crate::radians::Radians;
use crate::{Distance, Point};

use super::super::state::TurtleState;
use super::super::app::{TurtleId, TurtleDrawings};
use super::super::access_control::{AccessControl, RequiredData, RequiredTurtles};
use super::super::renderer::display_list::{DisplayList, PrimHandle};

pub(crate) async fn move_forward(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    id: TurtleId,
    distance: Distance,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, drawings} = turtles.one_mut();

    let &mut TurtleState {position, heading, ..} = turtle;

    // The total amount we'll move in the x and y directions
    let movement = Point {
        x: distance * heading.cos(),
        y: distance * heading.sin(),
    };
    let target_pos = position + movement;

    let mut display_list = display_list.lock().await;
    animate_movement(turtle, drawings, &mut display_list, target_pos).await;

    conn.send(client_id, ServerResponse::AnimationComplete(id)).await
        .expect("unable to send response to IPC client");
}

pub(crate) async fn move_to(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    id: TurtleId,
    target_pos: Point,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, drawings} = turtles.one_mut();

    let mut display_list = display_list.lock().await;
    animate_movement(turtle, drawings, &mut display_list, target_pos).await;

    conn.send(client_id, ServerResponse::AnimationComplete(id)).await
        .expect("unable to send response to IPC client");
}

async fn animate_movement(
    turtle: &mut TurtleState,
    drawings: &mut Vec<PrimHandle>,
    display_list: &mut DisplayList,
    target_pos: Point,
) {
    let &mut TurtleState {position, speed, ..} = turtle;

    //TODO
}

pub(crate) async fn rotate_in_place(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    id: TurtleId,
    angle: Radians,
    direction: RotationDirection,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, drawings} = turtles.one_mut();

    //TODO

    conn.send(client_id, ServerResponse::AnimationComplete(id)).await
        .expect("unable to send response to IPC client");
}
