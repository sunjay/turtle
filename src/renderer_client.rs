use std::sync::Arc;

use ipc_channel::ipc::IpcError;
use serde::{Serialize, Deserialize};
use tokio::sync::{mpsc, RwLock, Mutex};
use thiserror::Error;

use crate::ipc_protocol::{ClientConnection, ConnectionError, ClientRequest, ServerResponse};
use crate::renderer_server::RendererServer;

/// Signals that the IPC connection has been disconnected and therefore the window was probably
/// closed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("Cannot continue to run turtle commands after window is closed. This panic stops the thread, but is not necessarily an error.")]
struct Disconnected;

/// A unique ID used to multiplex responses on the client side
///
/// Treated as an opaque value on the server that is returned back to the client with the response
/// to a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(usize);

/// Spawns the server, and manages the IPC connection
///
/// Messages are sent though IPC and responses are dispatched back to the correct client based on
/// the received client ID.
#[derive(Debug)]
struct ClientDispatcher {
    /// The server task/process
    ///
    /// When dropped, this will block until the server process has quit. This field is explicitly
    /// owned by this struct and not reference counted in order to guarantee that this happens.
    server: RendererServer,

    /// The connection to the server task/process
    ///
    /// This will no longer send messages after the server process has terminated.
    conn: Arc<ClientConnection>,

    /// Each `ClientId` indexes into this field
    ///
    /// Using `RwLock` allows sending multiple times concurrently using `read()` and also allows
    /// more clients to be added using `write()`.
    clients: Arc<RwLock<Vec<mpsc::UnboundedSender<Result<ServerResponse, Disconnected>>>>>,
}

impl ClientDispatcher {
    async fn new() -> Result<Self, ConnectionError> {
        let (server, conn) = RendererServer::spawn().await?;
        let conn = Arc::new(conn);
        let clients = Arc::new(RwLock::new(Vec::<mpsc::UnboundedSender<_>>::new()));

        let task_conn = conn.clone();
        let task_clients = clients.clone();
        tokio::spawn(async move {
            loop {
                let (id, response) = match task_conn.recv().await {
                    Ok((id, response)) => (id, Ok(response)),

                    Err(IpcError::Disconnected) => {
                        // Alert all the clients of the disconnection
                        let clients = task_clients.read().await;
                        for client in &*clients {
                            match client.send(Err(Disconnected)) {
                                Ok(()) => {},
                                // This particular client connection must have gotten dropped
                                Err(_) => {},
                            }
                        }
                        break;
                    },

                    Err(err) => panic!("Error while receiving IPC message: {:?}", err),
                };

                let clients = task_clients.read().await;

                let ClientId(index) = id;
                match clients[index].send(response) {
                    Ok(()) => {},
                    // This particular client connection must have gotten dropped
                    Err(_) => {},
                }
            }
        });

        Ok(Self {server, conn, clients})
    }

    async fn add_client(&self) -> (ClientId, mpsc::UnboundedReceiver<Result<ServerResponse, Disconnected>>) {
        let mut clients = self.clients.write().await;

        let id = ClientId(clients.len());
        let (sender, receiver) = mpsc::unbounded_channel();
        clients.push(sender);

        (id, receiver)
    }

    async fn send(&self, id: ClientId, req: ClientRequest) -> Result<(), ipc_channel::Error> {
        self.conn.send(id, req).await
    }
}

/// Represents a single connection to the server
#[derive(Debug)]
pub struct RendererClient {
    dispatcher: Arc<ClientDispatcher>,
    id: ClientId,
    receiver: Mutex<mpsc::UnboundedReceiver<Result<ServerResponse, Disconnected>>>,
}

impl RendererClient {
    /// Spawns a new server process and creates a connection to it
    pub async fn new() -> Result<Self, ConnectionError> {
        let dispatcher = Arc::new(ClientDispatcher::new().await?);
        let (id, receiver) = dispatcher.add_client().await;
        let receiver = Mutex::new(receiver);

        Ok(Self {dispatcher, id, receiver})
    }

    /// Creates a new renderer client that can also communicate to the same server
    pub async fn split(&self) -> Self {
        let dispatcher = self.dispatcher.clone();
        let (id, receiver) = dispatcher.add_client().await;
        let receiver = Mutex::new(receiver);

        Self {dispatcher, id, receiver}
    }

    /// Sends a message to the server process
    ///
    /// When possible, prefer using methods from `ProtocolClient` instead of using this directly
    pub async fn send(&self, req: ClientRequest) {
        // The error produced by send is a serialization error, so it signals a bug in this code,
        // not something that should be propagated to be handled elsewhere.
        self.dispatcher.send(self.id, req).await
            .expect("bug: error while sending message through IPC")
    }

    /// Receives a response from the server process
    ///
    /// When possible, prefer using methods from `ProtocolClient` instead of using this directly
    pub async fn recv(&self) -> ServerResponse {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
            // Since this struct keeps a ref-counted copy of the senders, they can't have possibly
            // been dropped at this point.
            .expect("bug: client senders should not be dropped yet")
            // This panic causes the program to exit if turtle commands continue after the window
            // closes
            .unwrap_or_else(|err| panic!("IPC response not received: {}", err))
    }
}
