use std::thread;
use std::pin::Pin;
use std::task::{Poll, Context};

use tokio::sync::mpsc;
use tokio::stream::Stream;
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
    receiver: mpsc::UnboundedReceiver<Result<T, IpcError>>,
}

impl<T: Serialize + DeserializeOwned + Send + 'static> AsyncIpcReceiver<T> {
    pub fn new(ipc_receiver: IpcReceiver<T>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        thread::spawn(move || {
            loop {
                let next_value = match ipc_receiver.recv() {
                    // No longer connected
                    Err(IpcError::Disconnected) => break,
                    // For anything else, just forward along the entire value
                    value => value,
                };

                match sender.send(next_value) {
                    Ok(()) => {},
                    // Main thread has quit, so this thread should terminate too
                    Err(_) => break,
                }
            }
        });

        Self {receiver}
    }

    /// Receives the next value from the IPC receiver.
    pub async fn recv(&mut self) -> Result<T, IpcError> {
        // This should never return None because the spawned thread runs forever
        self.receiver.recv().await
            .expect("bug: thread carrying sender terminated before main thread")
    }
}

impl<T: Serialize + DeserializeOwned + Send + 'static> Stream for AsyncIpcReceiver<T> {
    type Item = Result<T, IpcError>;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.get_mut().receiver), ctx)
    }
}
