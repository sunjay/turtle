use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, Mutex};
use futures_util::future::{FutureExt, RemoteHandle};
use ipc_channel::ipc::IpcError;

use crate::renderer_client::ClientId;
use crate::ipc_protocol::{ClientConnection, ServerConnection, ConnectionError, ClientRequest, ServerResponse};

use super::super::{
    serve,
    app::App,
    renderer::display_list::DisplayList,
    test_event_loop_notifier::EventLoopNotifier
};

/// Spawns the task/process responsible for handling and responding to client requests
///
/// Also manages the client connection used for communicating with the server
#[derive(Debug)]
pub struct RendererServer {
    /// A remote handle is like a `JoinHandle` that drops its running task when it is dropped. A
    /// normal `JoinHandle` would detach the task, and that is not desirable for tests.
    task_handle: RemoteHandle<()>,
    /// The connection to the spawned sever process
    ///
    /// This will no longer send messages after the server process has terminated.
    conn: ClientConnection,
}

impl RendererServer {
    /// Runs any initialization logic required at the beginning of the program
    pub fn start() {}

    /// Spawns the backend in a new task and returns the struct that will be used to
    /// interface with it.
    pub async fn spawn() -> Result<Self, ConnectionError> {
        let (server_name_sender, server_name_receiver) = oneshot::channel();
        // Spawn a separate task for the server so this task can continue to make progress
        // while that runs. The remote handle will drop that future when it is dropped.
        let (child, task_handle) = async move {
            let server_name = server_name_receiver.await
                .expect("bug: unable to receive server name");
            run_main(server_name).await;
        }.remote_handle();

        tokio::spawn(child);

        let conn = ClientConnection::new(move |name| async {
            server_name_sender.send(name)
                .expect("bug: unable to send server name to test renderer server");
            Ok(())
        }).await?;

        Ok(Self {task_handle, conn})
    }

    /// Sends a request to the server
    pub async fn send(&self, id: ClientId, req: ClientRequest) -> Result<(), ipc_channel::Error> {
        self.conn.send(id, req).await
    }

    /// Receives a response from the server
    pub async fn recv(&self) -> Result<(ClientId, ServerResponse), IpcError> {
        self.conn.recv().await
    }
}

pub async fn run_main(server_name: String) {
    // The state of the drawing and the state/drawings associated with each turtle
    let app = Arc::new(App::default());
    // All of the drawing primitives in the order in which they wil be drawn
    //
    // This is managed separately from the rest of the app state because the display list is shared
    // among pretty much everything and so critical sections containing the display list need to be
    // as short as possible.
    let display_list = Arc::new(Mutex::new(DisplayList::default()));

    // Create the proxy that will be given to the thread managing IPC
    let event_loop_notifier = Arc::new(EventLoopNotifier::new());
    // A channel for transferring events
    let (_events_sender, events_receiver) = mpsc::unbounded_channel();

    let conn = ServerConnection::connect(server_name)
        .expect("unable to establish turtle server connection");
    serve(conn, app, display_list, event_loop_notifier, events_receiver).await;
}
