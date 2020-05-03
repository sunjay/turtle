mod state;
mod app;
mod access_control;
mod renderer;
mod handlers;
mod main;
mod start;

pub(crate) use app::TurtleId;
pub(crate) use renderer::export::ExportError;
pub use start::start;

use std::sync::Arc;

use glutin::event_loop::EventLoopProxy;
use tokio::{
    sync::Mutex,
    io::{self, AsyncBufReadExt},
};

use crate::ipc_protocol::{
    ServerConnection,
    ConnectionError,
    ClientRequest,
    ServerResponse,
};
use crate::renderer_client::ClientId;

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
    conn: ServerConnection,
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: EventLoopProxy<RequestRedraw>,
) -> ! {
    let conn = Arc::new(conn);
    let app_control = Arc::new(AccessControl::new(app).await);
    let event_loop = Arc::new(Mutex::new(event_loop));

    loop {
        let (client_id, request) = conn.recv().await
            .expect("unable to receive request from IPC client");

        // Each incoming request is given its own task configured specifically for each kind of
        // request. Having separate tasks allows requests that can run in parallel to do so.
        tokio::spawn(run_request(
            conn.clone(),
            client_id,
            app_control.clone(),
            display_list.clone(),
            event_loop.clone(),
            request,
        ));
    }
}

async fn run_request(
    conn: Arc<ServerConnection>,
    client_id: ClientId,
    app_control: Arc<AccessControl>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: Arc<Mutex<EventLoopProxy<RequestRedraw>>>,
    request: ClientRequest,
) {
    use ClientRequest::*;
    match request {
        CreateTurtle => {
            let id = app_control.add_turtle().await;
            conn.send(client_id, ServerResponse::NewTurtle(id)).await
                .expect("unable to send IPC response");
        },

        Export(path, format) => {
            handlers::export_drawings(&conn, client_id, &app_control, &display_list, &path, format).await;
        },

        NextEvent => {
            todo!()
        },

        DrawingProp(prop) => {
            handlers::drawing_prop(&conn, client_id, &app_control, prop).await
        },
        SetDrawingProp(prop_value) => {
            handlers::set_drawing_prop(&app_control, prop_value).await
        },

        TurtleProp(id, prop) => {
            handlers::turtle_prop(&conn, client_id, &app_control, id, prop).await
        },
        SetTurtleProp(id, prop_value) => {
            handlers::set_turtle_prop(&app_control, id, prop_value).await
        },

        _ => todo!()
    }
}
