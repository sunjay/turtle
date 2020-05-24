//! Handles the IPC (Inter-process Communication) protocol used by the renderer client and server.
//!
//! This is the common "language" between client/server. It defines the protocol for connection and
//! for sending messages back and forth.

mod async_ipc_receiver;
mod messages;
mod protocol;

pub use messages::*;
pub use protocol::*;

use std::io;
use std::future::Future;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcError};

use crate::renderer_client::ClientId;

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

/// The sender for the client side of the IPC connection
#[derive(Debug, Clone)]
pub struct ClientSender {
    sender: IpcSender<(ClientId, ClientRequest)>,
}

impl ClientSender {
    /// Sends a request to the server via IPC
    pub fn send(&self, id: ClientId, req: ClientRequest) -> Result<(), ipc_channel::Error> {
        self.sender.send((id, req))
    }
}

/// The receiver for the client side of the IPC connection
#[derive(Debug)]
pub struct ClientReceiver {
    receiver: AsyncIpcReceiver<HandshakeResponse>,
}

impl ClientReceiver {
    /// Waits for a response from the server via IPC
    pub async fn recv(&self) -> Result<(ClientId, ServerResponse), IpcError> {
        let response = self.receiver.recv().await?;
        match response {
            HandshakeResponse::Response(id, response) => Ok((id, response)),
            _ => unreachable!("bug: server did not send response after request"),
        }
    }
}

/// Establishes the client side of the IPC connection by providing a oneshot server name and
/// completing the handshake
pub async fn connect_client<S, F>(
    send_ipc_oneshot_name: S,
) -> Result<(ClientSender, ClientReceiver), ConnectionError>
    where S: FnOnce(String) -> F,
          F: Future<Output=io::Result<()>>,
{
    // Send the oneshot token to the server which will then respond with its own oneshot token
    let (server, server_name) = IpcOneShotServer::new()?;
    send_ipc_oneshot_name(server_name).await?;

    let (receiver, response): (_, HandshakeResponse) = tokio::task::spawn_blocking(|| {
        server.accept()
    }).await??;

    let sender = match response {
        HandshakeResponse::HandshakeFinish(sender) => sender,
        _ => unreachable!("bug: server did not send back Sender at the end of handshake"),
    };

    let sender = ClientSender {sender};
    let receiver = ClientReceiver {receiver: AsyncIpcReceiver::new(receiver)};

    Ok((sender, receiver))
}

/// Provides the ability to send a single response to a client
#[derive(Debug)]
pub struct ServerOneshotSender {
    client_id: ClientId,
    sender: ServerSender,
}

impl ServerOneshotSender {
    pub fn new(client_id: ClientId, sender: ServerSender) -> Self {
        Self {client_id, sender}
    }

    /// Sends a response to the client
    ///
    /// This method can only be called once and thus it ensures that every request is only
    /// responded to a single time
    pub fn send(self, res: ServerResponse) -> Result<(), ipc_channel::Error> {
        self.sender.send(self.client_id, res)
    }
}

/// The sender for the server side of the IPC connection
#[derive(Debug, Clone)]
pub struct ServerSender {
    sender: IpcSender<HandshakeResponse>,
}

impl ServerSender {
    /// Sends a response to the client
    ///
    /// This should only ever be done in response to a request
    pub fn send(&self, id: ClientId, res: ServerResponse) -> Result<(), ipc_channel::Error> {
        self.sender.send(HandshakeResponse::Response(id, res))
    }
}

/// The receiver for the server side of the IPC connection
#[derive(Debug)]
pub struct ServerReceiver {
    receiver: AsyncIpcReceiver<(ClientId, ClientRequest)>,
}

impl ServerReceiver {
    /// Returns the next request, waiting until one is available
    pub async fn recv(&self) -> Result<(ClientId, ClientRequest), IpcError> {
        self.receiver.recv().await
    }
}

/// Establishes a connection to the client by reading from stdin
#[cfg(all(not(any(feature = "test", test)), target_os = "macos"))]
pub async fn connect_server_stdin() -> Result<(ServerSender, ServerReceiver), ConnectionError> {
    use tokio::io::{self, AsyncBufReadExt};

    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);

    let mut oneshot_name = String::new();
    reader.read_line(&mut oneshot_name).await?;
    assert!(!oneshot_name.is_empty(), "bug: unexpected EOF when reading oneshot server name");

    // Remove the trailing newline
    assert_eq!(oneshot_name.pop(), Some('\n'));
    let conn = connect_server(oneshot_name)?;

    Ok(conn)
}

/// Establishes a connection with the IPC channel oneshot server with the given name
pub fn connect_server(
    oneshot_name: String,
) -> Result<(ServerSender, ServerReceiver), ConnectionError> {
    let (server_sender, receiver) = ipc::channel()?;
    let sender = IpcSender::connect(oneshot_name)?;

    // Finish handshake by giving client a sender it can use to send messages to the server
    sender.send(HandshakeResponse::HandshakeFinish(server_sender))?;

    let sender = ServerSender {sender};
    let receiver = ServerReceiver {receiver: AsyncIpcReceiver::new(receiver)};

    Ok((sender, receiver))
}
