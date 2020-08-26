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
    access_control::AccessControl,
};

pub(crate) async fn debug_turtle(
    data_req_queued: oneshot::Sender<()>,
    conn: ServerOneshotSender,
    app_control: &AccessControl,
    id: TurtleId,
    angle_unit: AngleUnit,
) -> Result<(), HandlerError> {
    let turtle = app_control.get(id, data_req_queued).await;
    let turtle = turtle.lock().await;

    let TurtleDrawings {state: turtle, ..} = &*turtle;

    let debug_state = turtle.to_debug(angle_unit);

    conn.send(ServerResponse::DebugTurtle(id, debug_state))?;

    Ok(())
}

pub(crate) async fn debug_drawing(
    data_req_queued: oneshot::Sender<()>,
    conn: ServerOneshotSender,
    app_control: &AccessControl,
) -> Result<(), HandlerError> {
    let drawing = app_control.get_drawing(data_req_queued).await;
    let drawing = drawing.lock().await;

    let debug_state = drawing.to_debug();

    conn.send(ServerResponse::DebugDrawing(debug_state))?;

    Ok(())
}
