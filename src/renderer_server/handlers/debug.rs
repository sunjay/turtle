use tokio::sync::oneshot;

use crate::{
    async_turtle::AngleUnit,
    ipc_protocol::{
        ServerOneshotSender,
        ServerResponse,
    },
};

use super::HandlerError;
use super::super::{
    app::{TurtleId, TurtleDrawings},
    access_control::{AccessControl, RequiredData, RequiredTurtles},
};

pub(crate) async fn debug_turtle(
    data_req_queued: oneshot::Sender<()>,
    conn: ServerOneshotSender,
    app_control: &AccessControl,
    id: TurtleId,
    angle_unit: AngleUnit,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }, data_req_queued).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, ..} = turtles.one_mut();

    let debug_state = turtle.to_debug(angle_unit);

    conn.send(ServerResponse::DebugTurtle(id, debug_state))?;

    Ok(())
}

pub(crate) async fn debug_drawing(
    data_req_queued: oneshot::Sender<()>,
    conn: ServerOneshotSender,
    app_control: &AccessControl,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }, data_req_queued).await;
    let drawing = data.drawing_mut();

    let debug_state = drawing.to_debug();

    conn.send(ServerResponse::DebugDrawing(debug_state))?;

    Ok(())
}
