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
use tokio::sync::Mutex;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcError};

use crate::renderer_client::{ClientId, RendererServerProcess};

use async_ipc_receiver::AsyncIpcReceiver;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ConnectionError {
    IpcChannelError(#[from] ipc_channel::Error),
    IOError(#[from] io::Error),
    JoinError(#[from] tokio::task::JoinError),
}

/// Represents either a sender from the connection handshake or a response
#[derive(Debug, Serialize, Deserialize)]
enum HandshakeResponse {
    /// Represents that the handshake has been completed successfully.
    ///
    /// Provides the sender that the client should use to send further requests to the server. This
    /// message is only ever sent **once** for each client at the beginning when the connection is
    /// first established.
    HandshakeFinish(IpcSender<(ClientId, ClientRequest)>),

    /// A response from the server sent in response to a request
    Response(ClientId, ServerResponse),
}

/// Represents the client side of the IPC connection
#[derive(Debug)]
pub struct ClientConnection {
    sender: IpcSender<(ClientId, ClientRequest)>,
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

    /// Sends a request to the server via IPC
    pub fn send(&self, id: ClientId, req: ClientRequest) -> Result<(), ipc_channel::Error> {
        self.sender.send((id, req))
    }

    /// Waits for a response from the server via IPC
    pub async fn recv(&self) -> Result<(ClientId, ServerResponse), IpcError> {
        let response = self.receiver.recv().await?;
        match response {
            HandshakeResponse::Response(id, response) => Ok((id, response)),
            _ => unreachable!("bug: server did not send response after request"),
        }
    }
}

/// Represents the server side of the IPC connection
#[derive(Debug)]
pub struct ServerConnection {
    sender: Mutex<IpcSender<HandshakeResponse>>,
    receiver: AsyncIpcReceiver<(ClientId, ClientRequest)>,
}

impl ServerConnection {
    /// Establishes a connection to the client by reading from stdin
    pub async fn connect_stdin() -> Result<Self, ConnectionError> {
        use tokio::io::{self, AsyncBufReadExt};

        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin);

        let mut oneshot_name = String::new();
        reader.read_line(&mut oneshot_name).await?;
        if oneshot_name.is_empty() {
            panic!("bug: unexpected EOF when reading oneshot server name");
        }

        // Remove the trailing newline
        assert_eq!(oneshot_name.pop(), Some('\n'));
        let conn = ServerConnection::connect(oneshot_name)?;

        Ok(conn)
    }

    /// Establishes a connection with the IPC channel oneshot server with the given name
    pub fn connect(oneshot_name: String) -> Result<Self, ConnectionError> {
        let (server_sender, receiver) = ipc::channel()?;
        let sender = IpcSender::connect(oneshot_name)?;

        // Finish handshake by giving client a sender it can use to send messages to the server
        sender.send(HandshakeResponse::HandshakeFinish(server_sender))?;

        let sender = Mutex::new(sender);
        let receiver = AsyncIpcReceiver::new(receiver);

        Ok(Self {sender, receiver})
    }

    /// Returns the next request, waiting until one is available
    pub async fn recv(&self) -> Result<(ClientId, ClientRequest), IpcError> {
        self.receiver.recv().await
    }

    /// Sends a response to the client
    ///
    /// This should only ever be done in response to a request
    pub async fn send(&self, id: ClientId, res: ServerResponse) -> Result<(), ipc_channel::Error> {
        self.sender.lock().await
            .send(HandshakeResponse::Response(id, res))
    }
}
