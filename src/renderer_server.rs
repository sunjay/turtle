mod state;
mod app;
mod access_control;
mod renderer;
mod main;
mod start;

pub(crate) use app::TurtleId;
pub use start::start;

use std::sync::Arc;

use glutin::event_loop::EventLoopProxy;
use tokio::{
    sync::Mutex,
    io::{self, AsyncBufReadExt},
};

use crate::ipc_protocol::{ServerConnection, ConnectionError, ClientRequest, ServerResponse};

use app::App;
use access_control::AccessControl;
use renderer::display_list::DisplayList;

/// A custom event used to tell the glutin event loop to redraw the window
#[derive(Debug, Clone, PartialEq, Eq)]
struct RequestRedraw;

/// Establishes a connection to the client by reading from stdin
async fn connect() -> Result<ServerConnection, ConnectionError> {
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

/// Serves requests from the client forever
async fn serve(
    mut conn: ServerConnection,
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: EventLoopProxy<RequestRedraw>,
) -> ! {
    let app_control = AccessControl::new(app).await;

    loop {
        let (client_id, request) = conn.recv().await
            .expect("unable to receive request from IPC client");

        use ClientRequest::*;
        match request {
            CreateTurtle => {
                let id = app_control.add_turtle().await;
                conn.send(client_id, ServerResponse::NewTurtle(id))
                    .expect("unable to send IPC response");
            },

            _ => todo!()
        }
    }
}
