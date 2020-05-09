mod renderer_server_process;

pub use renderer_server_process::*;

use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::{mpsc, RwLock, Mutex};

use crate::ipc_protocol::{ClientConnection, ConnectionError, ClientRequest, ServerResponse};

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
    /// The spawned server process
    ///
    /// When dropped, this will block until the server process has quit
    proc: RendererServerProcess,

    /// The IPC connection to the server process
    conn: Arc<ClientConnection>,

    /// Each `ClientId` indexes into this field
    ///
    /// Using `RwLock` allows sending multiple times concurrently using `read()` and also allows
    /// more clients to be added using `write()`.
    clients: Arc<RwLock<Vec<mpsc::UnboundedSender<ServerResponse>>>>,
}

impl ClientDispatcher {
    async fn new() -> Result<Self, ConnectionError> {
        let mut proc = RendererServerProcess::spawn().await?;
        let conn = Arc::new(ClientConnection::new(&mut proc).await?);
        let clients = Arc::new(RwLock::new(Vec::<mpsc::UnboundedSender<_>>::new()));

        let task_conn = conn.clone();
        let task_clients = clients.clone();
        tokio::spawn(async move {
            loop {
                let (id, response) = task_conn.recv().await
                    .expect("server process shutdown");
                let clients = task_clients.read().await;

                let ClientId(index) = id;
                clients[index].send(response)
                    .expect("server process shutdown");
            }
        });

        Ok(Self {proc, conn, clients})
    }

    async fn add_client(&self) -> (ClientId, mpsc::UnboundedReceiver<ServerResponse>) {
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
    receiver: Mutex<mpsc::UnboundedReceiver<ServerResponse>>,
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
    pub async fn send(&self, req: ClientRequest) -> Result<(), ipc_channel::Error> {
        self.dispatcher.send(self.id, req).await
    }

    /// Receives a response from the server process
    ///
    /// When possible, prefer using methods from `ProtocolClient` instead of using this directly
    pub async fn recv(&self) -> ServerResponse {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
            .expect("bug: unable to receive response from server process")
    }
}
