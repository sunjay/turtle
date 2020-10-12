use crate::{
    async_turtle::AngleUnit,
    ipc_protocol::{ServerOneshotSender, ServerResponse},
};

use super::super::app::{App, TurtleDrawings, TurtleId};
use super::HandlerError;

pub(crate) fn debug_turtle(
    conn: ServerOneshotSender,
    app: &App,
    id: TurtleId,
    angle_unit: AngleUnit,
) -> Result<(), HandlerError> {
    let turtle = app.turtle(id);

    let TurtleDrawings { state: turtle, .. } = turtle;

    let debug_state = turtle.to_debug(angle_unit);

    conn.send(ServerResponse::DebugTurtle(id, debug_state))?;

    Ok(())
}

pub(crate) fn debug_drawing(conn: ServerOneshotSender, app: &App) -> Result<(), HandlerError> {
    let drawing = app.drawing();

    let debug_state = drawing.to_debug();

    conn.send(ServerResponse::DebugDrawing(debug_state))?;

    Ok(())
}
