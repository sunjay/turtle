use crate::ipc_protocol::{ServerOneshotSender, ServerResponse};

use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    app::App,
};

pub(crate) fn create_turtle(
    conn: ServerOneshotSender,
    app: &mut App,
    event_loop: &EventLoopNotifier,
) -> Result<(), HandlerError> {
    let id = app.add_turtle();

    // Signal the main thread that the image has changed (otherwise the new turtle won't be drawn)
    event_loop.request_redraw()?;

    conn.send(ServerResponse::NewTurtle(id))?;

    Ok(())
}
