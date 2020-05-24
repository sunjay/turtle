use crate::ipc_protocol::{ServerOneshotSender, ServerResponse};

use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    access_control::AccessControl,
};

pub(crate) async fn create_turtle(
    conn: ServerOneshotSender,
    app_control: &AccessControl,
    event_loop: EventLoopNotifier,
) -> Result<(), HandlerError> {
    // Creating a new turtle doesn't require locking any data because it is impossible for any
    // future request to depend on a turtle that doesn't exist yet
    let id = app_control.add_turtle().await;

    // Signal the main thread that the image has changed
    event_loop.request_redraw()?;

    conn.send(ServerResponse::NewTurtle(id))?;

    Ok(())
}
