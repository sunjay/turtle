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

                // NOTE: If the future from `AsyncIpcReceiver::recv` is dropped before the next
                // value is sent, the next value will be received by the next caller of
                // `AsyncIpcReceiver::recv`.
                //
                // A way to fix this is to have `AsyncIpcReceiver::recv` send a oneshot channel to
                // this thread. That oneshot channel will be sent the next value, thus guaranteeing
                // that each call to `recv` gets the next value from `ipc_receiver.recv()`.
                //
                // While this consistency is nice, it was decided to leave the implementation
                // simple for now. That means the semantics of `AsyncIpcReceiver` are closer to a
                // single producer/single consumer system with a queue, rather than the single
                // channel that it is meant to model. This is (probably) fine given that this API
                // is only used in a single producer/single consumer context.
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
    ///
    /// NOTE: Usually, a channel API guarantees that each call to recv will get the next value from
    /// the channel. While this is still the case here under normal usage, if the future returned
    /// from this method is dropped before being completed, the next call to this method will get
    /// the value that the *previous* call would have received. This is closer to the semantics of
    /// a single producer/single consumer with a queue. If those semantics pose a problem, make
    /// sure that the future is not dropped before being completed!
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
