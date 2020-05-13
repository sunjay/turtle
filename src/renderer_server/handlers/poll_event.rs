use tokio::sync::{
    Mutex,
    mpsc::{self, error::TryRecvError},
};

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
};
use crate::renderer_client::ClientId;
use crate::Event;

use super::HandlerError;

pub(crate) async fn poll_event(
    conn: &ServerConnection,
    client_id: ClientId,
    events_receiver: &Mutex<mpsc::UnboundedReceiver<Event>>,
) -> Result<(), HandlerError> {
    let mut events_receiver = events_receiver.lock().await;

    let event = match events_receiver.try_recv() {
        Ok(event) => Some(event),
        Err(TryRecvError::Empty) => None,
        // The main thread must have ended so no more events will be sent ever
        Err(TryRecvError::Closed) => return Ok(()),
    };

    conn.send(client_id, ServerResponse::Event(event)).await?;

    Ok(())
}
