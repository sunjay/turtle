use std::thread;

use tokio::sync::{mpsc, oneshot};
use tokio::runtime::Handle;
use serde::{Serialize, de::DeserializeOwned};
use ipc_channel::ipc::{IpcReceiver, IpcError};

/// Asynchronous wrapper over `IpcReceiver` that yields `Result<T, IpcError>`.
///
/// The `ipc_channel::ipc::IpcReceiver` type can't be shared between threads, so it can't be used
/// from `tokio::task::spawn_blocking`. This type uses a separate thread to manage receiving
/// values asynchronously. It implements the `Stream` trait, which allows it to be polled over
/// and over again for each received value.
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
                let next_value = match ipc_receiver.recv() {
                    // No longer connected
                    Err(IpcError::Disconnected) => break,
                    // For anything else, just forward along the entire value
                    value => value,
                };

                let value_sender: oneshot::Sender<_> = match handle.block_on(receiver.recv()) {
                    Some(value_sender) => value_sender,
                    // Main thread has quit, so this thread should terminate too
                    None => break,
                };
                match value_sender.send(next_value) {
                    Ok(()) => {}, // Sent successfully
                    Err(_) => {}, // Future was dropped, so this value will not be sent
                }
            }
        });

        Self {channel_sender}
    }

    /// Receives the next value from the IPC receiver
    pub async fn recv(&self) -> Result<T, IpcError> {
        let (sender, receiver) = oneshot::channel();

        // The channels should never return errors because the spawned thread runs forever
        self.channel_sender.send(sender)
            .unwrap_or_else(|_| panic!("bug: thread managing IPC receiver terminated before main thread"));

        receiver.await
            .expect("bug: thread managing IPC receiver terminated before main thread")
    }
}
