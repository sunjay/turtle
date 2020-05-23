use crate::ipc_protocol::{ServerConnection, ServerResponse};
use crate::renderer_client::ClientId;

use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    access_control::AccessControl,
};

pub(crate) async fn create_turtle(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    event_loop: EventLoopNotifier,
) -> Result<(), HandlerError> {
    // Creating a new turtle doesn't require locking any data because it is impossible for any
    // future request to depend on a turtle that doesn't exist yet
    let id = app_control.add_turtle().await;

    // Signal the main thread that the image has changed
    event_loop.request_redraw()?;

    conn.send(client_id, ServerResponse::NewTurtle(id)).await?;

    Ok(())
}
