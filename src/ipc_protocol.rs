//! Handles the IPC (Inter-process Communication) protocol used by the renderer client and server.
//!
//! This is the common "language" between client/server. It defines both the protocol for
//! connection and for send messages back and forth.

mod async_ipc_receiver;
mod messages;

pub use messages::*;

use std::io;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcError};

use crate::renderer_client::RendererServerProcess;

use async_ipc_receiver::AsyncIpcReceiver;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ConnectionError {
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
    IpcError(IpcError),
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

/// Represents the client side of the IPC connection
#[derive(Debug)]
pub struct ClientConnection {
    sender: IpcSender<ClientRequest>,
    receiver: AsyncIpcReceiver<HandshakeResponse>,
}

impl ClientConnection {
    pub async fn new(process: &mut RendererServerProcess) -> Result<Self, ConnectionError> {
        // Send the oneshot token to the server which will then respond with its own oneshot token
        let (server, server_name) = IpcOneShotServer::new()?;
        process.writeln(server_name).await?;

        let (receiver, response): (_, HandshakeResponse) = tokio::task::spawn_blocking(|| {
            server.accept()
        }).await??;

        let sender = match response {
            HandshakeResponse::HandshakeFinish(sender) => sender,
            _ => unreachable!("bug: server did not send back Sender at the end of handshake"),
        };
        let receiver = AsyncIpcReceiver::new(receiver);

        Ok(Self {sender, receiver})
    }

    /// Sends a request and awaits the response
    pub async fn send(&mut self, req: ClientRequest) -> Result<ServerResponse, SendError> {
        self.sender.send(req)?;
        let response = self.receiver.recv().await?;
        match response {
            HandshakeResponse::Response(response) => Ok(response),
            _ => unreachable!("bug: server did not send response after request"),
        }
    }
}

/// Represents the server side of the IPC connection
#[derive(Debug)]
pub struct ServerConnection {
    sender: IpcSender<HandshakeResponse>,
    receiver: AsyncIpcReceiver<ClientRequest>,
}

impl ServerConnection {
    /// Establishes a connection with the IPC channel one shot server with the given name
    pub fn connect(one_shot_name: String) -> Result<Self, ConnectionError> {
        let (server_sender, receiver) = ipc::channel()?;
        let sender = IpcSender::connect(one_shot_name)?;

        // Finish handshake by giving client a sender it can use to send messages to the server
        sender.send(HandshakeResponse::HandshakeFinish(server_sender))?;

        let receiver = AsyncIpcReceiver::new(receiver);

        Ok(Self {sender, receiver})
    }

    /// Returns the next request, waiting until one is available
    pub async fn recv(&mut self) -> Result<ClientRequest, IpcError> {
        self.receiver.recv().await
    }

    /// Sends a response to the client
    ///
    /// This should only ever be done in response to a request
    pub fn send(&mut self, res: ServerResponse) -> Result<(), ipc_channel::Error> {
        self.sender.send(HandshakeResponse::Response(res))
    }
}
