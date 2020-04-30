mod start;
mod main;

pub use start::start;

use tokio::io::{self, AsyncBufReadExt};

use crate::ipc_protocol::{ServerConnection, ConnectionError};

/// Manages one or more connections to renderer clients
#[derive(Debug)]
struct RendererServer {
    conn: ServerConnection,
}

impl RendererServer {
    /// Establishes a connection to the client by reading from stdin
    pub async fn new() -> Result<Self, ConnectionError> {
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

        Ok(Self {conn})
    }

    /// Serves requests from the client forever
    pub fn serve() -> ! {
        loop {}
    }
}
