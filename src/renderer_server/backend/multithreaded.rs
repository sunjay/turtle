use std::panic;

use tokio::{
    task,
    sync::oneshot,
    runtime::Handle,
};
use futures_util::future::{FutureExt, RemoteHandle};

use crate::ipc_protocol::{ClientConnection, ServerConnection, ConnectionError};

use super::super::main::run_main;

/// Spawns the task responsible for handling and responding to client requests
#[derive(Debug)]
pub struct RendererServer {
    /// A handle to the runtime that the task was spawned in. This is needed because a handle
    /// to the runtime can only be created within a "runtime context". Since Drop may not always
    /// run from async code, we need this to ensure we can wait on the task in `task_handle`.
    /// NOTE: This creates an implicit invariant that this struct must be dropped before the
    /// runtime that it was created in is dropped. This is not an issue in normal code and will at
    /// worst cause a panic!().
    runtime_handle: Handle,
    /// A handle to the running task. This can be waited on to find out if the window exited
    /// successfully. A remote handle will also drop the future it is associated with when it is
    /// dropped. (unlike a `JoinHandle` which will detach instead.) This is important to make sure
    /// the window closes when the thread holding this struct panics.
    task_handle: Option<RemoteHandle<Result<(), task::JoinError>>>,
}

impl RendererServer {
    /// Runs any initialization logic required at the beginning of the program
    pub fn start() {}

    /// Spawns the backend in a new task and returns the struct that will be used to
    /// interface with it.
    pub async fn spawn() -> Result<(Self, ClientConnection), ConnectionError> {
        let (server_name_sender, server_name_receiver) = oneshot::channel();
        // Spawn a separate task for the server so this task can continue to make progress
        // while that runs. The remote handle will drop that future when it is dropped.
        let (child, task_handle) = async move {
            let server_name = server_name_receiver.await
                .expect("bug: unable to receive server name");
            let handle = Handle::current();
            // spawn_blocking() takes care of catching any panics that might occur, so we don't
            // need to do that explicitly here even though Drop will need that information.
            task::spawn_blocking(|| {
                run_main(handle, async { ServerConnection::connect(server_name) })
            }).await
        }.remote_handle();

        tokio::spawn(child);

        let runtime_handle = Handle::current();
        let task_handle = Some(task_handle);

        let conn = ClientConnection::new(move |name| async {
            server_name_sender.send(name)
                .expect("bug: unable to send server name to renderer server");
            Ok(())
        }).await?;

        Ok((Self {runtime_handle, task_handle}, conn))
    }
}

impl Drop for RendererServer {
    fn drop(&mut self) {
        use std::thread;

        // If the current thread is panicking, we don't want to wait for the window to be closed
        // since the user likely has an issue to fix in their program.
        if thread::panicking() {
            // The `RemoteHandle` will take care of killing the task (a `JoinHandle` would detach)
            return;
        }

        // If this is just a normal ending of the main thread, we want to leave the renderer
        // running so that the user can see their drawing as long as they keep the window open

        // This unwrap is safe because no struct gets dropped twice
        let task_handle = self.task_handle.take().unwrap();

        // Wait for the task running the window to finish
        match self.runtime_handle.block_on(task_handle) {
            // Exit normally
            Ok(()) => {},
            // Propagate the panic
            //
            // `into_panic()` can fail if the task was cancelled instead of panicking, but that can
            // only happen if the runtime shuts down before this Drop impl is run. This cannot be
            // the case though because we explicitly assume that the runtime shuts down after this.
            // If that was not the case, the `block_on` call above would fail.
            Err(panic_err) => panic::resume_unwind(panic_err.into_panic()),
        }
    }
}
