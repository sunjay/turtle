//! Handles the IPC (Inter-process Communication) protocol used by the renderer client and server.
//!
//! This is the common "language" between client/server. It defines both the protocol for
//! connection and for send messages back and forth.

mod messages;

pub use messages::*;

use std::io;
use std::thread;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use tokio::sync::mpsc;

use crate::renderer_server::RendererServerProcess;

/// The environment variable that is set to indicate that the current process is a server process
pub const RENDERER_PROCESS_ENV_VAR: &str = "RUN_TURTLE_CANVAS";

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ClientConnectionError {
    IpcChannelError(#[from] ipc_channel::Error),
    IOError(#[from] io::Error),
    JoinError(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SendError {
    IpcChannelError(#[from] ipc_channel::Error),
    // IpcError does not implement Display for some reason...
    #[error("IPC Error: {0:?}")]
    IpcError(ipc_channel::ipc::IpcError),
}

impl From<ipc_channel::ipc::IpcError> for SendError {
    fn from(err: ipc_channel::ipc::IpcError) -> Self {
        SendError::IpcError(err)
    }
}

/// Represents either a sender from the connection handshake or a response
#[derive(Debug, Serialize, Deserialize)]
enum HandshakeResponse {
    /// Represents that the handshake has been completed successfully.
    ///
    /// Provides the sender that the client should use to send further requests to the server. This
    /// message is only ever sent **once** for each client at the beginning when the connection is
    /// first established.
    HandshakeFinish(IpcSender<ClientRequest>),

    /// A response from the server sent in response to a request
    Response(ServerResponse),
}

/// Represents one end of an IPC connection
///
/// The connection is parameterized by the type of message that will be sent from this end and
/// received from the other end.
pub struct ClientConnection {
    sender: mpsc::UnboundedSender<ClientRequest>,
    receiver: mpsc::UnboundedReceiver<Result<HandshakeResponse, SendError>>,
}

impl ClientConnection {
    pub async fn new(process: &mut RendererServerProcess) -> Result<Self, ClientConnectionError> {
        // Send the oneshot token to the server which will then respond with its own oneshot token
        let (server, server_name) = tokio::task::spawn_blocking(IpcOneShotServer::new).await??;
        process.writeln(server_name).await?;

        let (server_receiver, response): (_, HandshakeResponse) = tokio::task::spawn_blocking(|| {
            server.accept()
        }).await??;

        let server_sender = match response {
            HandshakeResponse::HandshakeFinish(sender) => sender,
            _ => unreachable!("bug: server did not send back Sender at the end of handshake"),
        };

        // IpcSender and IpcReceiver can't be shared between threads, so we have to spawn our own
        // thread to be able to use them asynchronously.
        let (sender, mut request_receiver) = mpsc::unbounded_channel();
        let (response_sender, receiver) = mpsc::unbounded_channel();
        thread::spawn(move || {
            // This isn't a thread managed by tokio, so it seems like using
            // tokio::task::block_in_place would be inappropriate. Using `block_on` instead, though
            // I'm not sure if this will accidentally spawn a second executor...
            use futures::executor::block_on;

            loop {
                let req = block_on(request_receiver.recv())
                    .expect("bug: the main thread exited earlier than expected");
                // Send the request to the server, then wait for the response
                let response = server_sender.send(req)
                    .map_err(SendError::from)
                    .and_then(|()| server_receiver.recv().map_err(SendError::from));

                response_sender.send(response)
                    .expect("bug: the main thread exited earlier than expected");
            }
        });

        Ok(Self {sender, receiver})
    }

    pub async fn send(&mut self, req: ClientRequest) -> Result<ServerResponse, SendError> {
        self.sender.send(req).expect("bug: the server thread exited earlier than expected");
        let response = self.receiver.recv().await
            .expect("bug: the server thread exited earlier than expected")?;
        match response {
            HandshakeResponse::Response(response) => Ok(response),
            _ => unreachable!("bug: server did not send response after request"),
        }
    }
}
