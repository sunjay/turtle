mod state;
mod app;
mod renderer;
mod main;
mod start;

pub(crate) use app::TurtleId;
pub use start::start;

use glutin::event_loop::EventLoopProxy;
use tokio::io::{self, AsyncBufReadExt};

use crate::ipc_protocol::{ServerConnection, ConnectionError};

/// A custom event used to tell the glutin event loop to redraw the window
#[derive(Debug, Clone, PartialEq, Eq)]
struct RequestRedraw;

/// Manages one or more connections to renderer clients
#[derive(Debug)]
struct RendererServer {
    conn: ServerConnection,
    event_loop: EventLoopProxy<RequestRedraw>,
}

impl RendererServer {
    /// Establishes a connection to the client by reading from stdin
    pub async fn new(event_loop: EventLoopProxy<RequestRedraw>) -> Result<Self, ConnectionError> {
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

        Ok(Self {conn, event_loop})
    }

    /// Serves requests from the client forever
    pub async fn serve(&mut self) -> ! {
        loop {}
    }
}
