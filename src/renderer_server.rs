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
use tokio::sync::Mutex;

use crate::ipc_protocol::{
    ServerConnection,
    ClientRequest,
    ServerResponse,
};
use crate::renderer_client::ClientId;

use app::App;
use access_control::AccessControl;
use renderer::display_list::DisplayList;

/// A custom event used to tell the glutin event loop to redraw the window
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RequestRedraw;

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

        MoveForward(id, distance) => {
            handlers::move_forward(&conn, client_id, &app_control, &display_list, &event_loop, id, distance).await
        },
        MoveTo(id, target_pos) => {
            handlers::move_to(&conn, client_id, &app_control, &display_list, &event_loop, id, target_pos).await
        },
        RotateInPlace(id, angle, direction) => {
            handlers::rotate_in_place(&conn, client_id, &app_control, &event_loop, id, angle, direction).await
        },

        BeginFill(id) => {
            handlers::begin_fill(&app_control, &display_list, &event_loop, id).await
        },
        EndFill(id) => {
            handlers::end_fill(&app_control, id).await
        },

        _ => todo!()
    }
}
