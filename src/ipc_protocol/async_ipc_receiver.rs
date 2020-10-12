use std::thread;

use ipc_channel::ipc::{IpcError, IpcReceiver};
use serde::{de::DeserializeOwned, Serialize};
use tokio::runtime::Handle;
use tokio::sync::{mpsc, oneshot};

/// Asynchronous wrapper over `IpcReceiver` that yields `Result<T, IpcError>`.
///
/// The `ipc_channel::ipc::IpcReceiver` type can't be shared between threads, so it can't be used
/// from `tokio::task::spawn_blocking`. This type uses a separate thread to manage receiving
/// values asynchronously.
#[derive(Debug)]
pub struct AsyncIpcReceiver<T: Serialize + DeserializeOwned + Send + 'static> {
    /// Each AsyncIpcReceiver::recv() call sends a oneshot channel that collects the result when
    /// it is ready. This ensures proper ordering even if the future from `recv` is dropped and has
    /// the added benefit of allowing `recv()` to take `&self` instead of `&mut self`. The second
    /// benefit allows some operations that use this struct to be lock-free.
    channel_sender: mpsc::UnboundedSender<oneshot::Sender<Result<T, IpcError>>>,
}

impl<T: Serialize + DeserializeOwned + Send + 'static> AsyncIpcReceiver<T> {
    pub fn new(ipc_receiver: IpcReceiver<T>) -> Self {
        let (channel_sender, mut receiver) = mpsc::unbounded_channel();

        let handle = Handle::current();
        thread::spawn(move || {
            loop {
                // This loop continues even if this call produces an error saying that IPC was
                // disconnected. This allows callers of `recv` to detect the disconnection.
                let next_value = ipc_receiver.recv();

                let value_sender: oneshot::Sender<_> = match handle.block_on(receiver.recv()) {
                    Some(value_sender) => value_sender,
                    // The sender for this channel only drops when the main thread has stopped,
                    // so it's pretty safe to stop here
                    None => break,
                };
                // The send may fail if the future was dropped, but that is not a fatal error
                value_sender.send(next_value).unwrap_or(());
            }
        });

        Self { channel_sender }
    }

    /// Receives the next value from the IPC receiver
    pub async fn recv(&self) -> Result<T, IpcError> {
        let (sender, receiver) = oneshot::channel();

        // The channels should never return errors because the spawned thread runs as long as this
        // thread is still running
        self.channel_sender.send(sender).unwrap_or_else(|_| {
            panic!("bug: thread managing IPC receiver terminated before main thread")
        });

        receiver
            .await
            .expect("bug: thread managing IPC receiver terminated before main thread")
    }
}
