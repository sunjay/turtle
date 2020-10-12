use futures_util::future::{FutureExt, RemoteHandle};
use tokio::sync::{mpsc, oneshot};

use crate::ipc_protocol::{
    connect_client, connect_server, ClientReceiver, ClientSender, ConnectionError,
};

use super::super::{
    app::SharedApp, renderer::display_list::SharedDisplayList, serve,
    test_event_loop_notifier::EventLoopNotifier,
};

/// Spawns the task responsible for handling and responding to client requests
#[derive(Debug)]
pub struct RendererServer {
    /// A remote handle is like a `JoinHandle` that drops its running task when it is dropped. A
    /// normal `JoinHandle` would detach the task, and that is not desirable for tests.
    task_handle: RemoteHandle<()>,
}

impl RendererServer {
    /// Runs any initialization logic required at the beginning of the program
    pub fn start() {}

    /// Spawns the backend in a new task and returns the struct that will be used to
    /// interface with it.
    pub async fn spawn() -> Result<(Self, ClientSender, ClientReceiver), ConnectionError> {
        let (server_name_sender, server_name_receiver) = oneshot::channel();
        // Spawn a separate task for the server so this task can continue to make progress
        // while that runs. The remote handle will drop that future when it is dropped.
        let (child, task_handle) = async move {
            let server_name = server_name_receiver
                .await
                .expect("bug: unable to receive server name");
            run_main(server_name).await;
        }
        .remote_handle();

        tokio::spawn(child);

        let (conn_sender, conn_receiver) = connect_client(move |name| async {
            server_name_sender
                .send(name)
                .expect("bug: unable to send server name to test renderer server");
            Ok(())
        })
        .await?;

        Ok((Self { task_handle }, conn_sender, conn_receiver))
    }
}

pub async fn run_main(server_name: String) {
    // The state of the drawing and the state/drawings associated with each turtle
    let app = SharedApp::default();
    // All of the drawing primitives in the order in which they wil be drawn
    //
    // This is managed separately from the rest of the app state because the display list is shared
    // among pretty much everything and so critical sections containing the display list need to be
    // as short as possible.
    let display_list = SharedDisplayList::default();

    // Create the proxy that will be given to the thread managing IPC
    let event_loop_notifier = EventLoopNotifier::new();
    // A channel for transferring events
    let (_events_sender, events_receiver) = mpsc::unbounded_channel();
    // A channel for notifying on shutdown
    let (_server_shutdown, server_shutdown_receiver) = mpsc::channel(1);

    let (conn_sender, conn_receiver) =
        connect_server(server_name).expect("unable to establish turtle server connection");

    serve(
        conn_sender,
        conn_receiver,
        app,
        display_list,
        event_loop_notifier,
        events_receiver,
        server_shutdown_receiver,
    )
    .await;
}
