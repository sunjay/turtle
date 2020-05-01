mod state;
mod app;
mod renderer;
mod main;
mod start;

pub(crate) use app::TurtleId;
pub use start::start;

use std::sync::Arc;

use glutin::event_loop::EventLoopProxy;
use tokio::io::{self, AsyncBufReadExt};

use crate::ipc_protocol::{ServerConnection, ConnectionError, ClientRequest, ServerResponse};

use app::App;

/// A custom event used to tell the glutin event loop to redraw the window
#[derive(Debug, Clone, PartialEq, Eq)]
struct RequestRedraw;

/// Manages one or more connections to renderer clients
#[derive(Debug)]
struct RendererServer {
    app: Arc<App>,
    conn: ServerConnection,
    event_loop: EventLoopProxy<RequestRedraw>,
}

impl RendererServer {
    /// Establishes a connection to the client by reading from stdin
    pub async fn new(app: Arc<App>, event_loop: EventLoopProxy<RequestRedraw>) -> Result<Self, ConnectionError> {
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

        Ok(Self {app, conn, event_loop})
    }

    /// Serves requests from the client forever
    pub async fn serve(&mut self) -> ! {
        loop {
            let (client_id, request) = self.conn.recv().await
                .expect("unable to receive request from IPC client");

            use ClientRequest::*;
            match request {
                CreateTurtle => {
                    let id = self.app.add_turtle().await;
                    self.conn.send(client_id, ServerResponse::NewTurtle(id))
                        .expect("unable to send IPC response");
                },

                _ => todo!()
            }
        }
    }
}
