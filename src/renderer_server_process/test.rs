use std::io;

use tokio::sync::mpsc;
use futures_util::future::{FutureExt, RemoteHandle};

use crate::renderer_server;

#[derive(Debug)]
pub struct RendererServerProcess {
    /// A remote handle is like a `JoinHandle` that drops its running task when it is dropped. A
    /// normal `JoinHandle` would detach the task, and that is not desirable for tests.
    task_handle: RemoteHandle<()>,
    /// A channel used to send the server name for the IPC connection
    ///
    /// Could use a oneshot channel for this, but that makes the implementation a bit messier and
    /// isn't really worth it.
    server_name_sender: mpsc::Sender<String>,
}

impl RendererServerProcess {
    /// For testing purposes. Instead of a process, this spawns a task from within the runtime
    pub fn spawn() -> io::Result<Self> {
        let (server_name_sender, mut server_name_receiver) = mpsc::channel(1);
        // Spawn a separate task for the server so this task can continue to make progress
        // while that runs. The remote handle will drop that future when it is dropped.
        let (child, task_handle) = async move {
            let server_name = server_name_receiver.recv().await
                .expect("bug: unable to receive server name");
            renderer_server::main(server_name).await;
        }.remote_handle();
        tokio::spawn(child);

        Ok(Self {task_handle, server_name_sender})
    }

    /// Provides the server name to the server
    pub async fn writeln(&mut self, data: String) -> io::Result<()> {
        self.server_name_sender.send(data).await
            .expect("bug: unable to send server name to test renderer server");

        Ok(())
    }
}
