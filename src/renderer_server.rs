mod state;
mod app;
mod access_control;
mod renderer;
mod event_loop_notifier;
mod handlers;
mod main;
mod start;

pub(crate) use app::TurtleId;
pub use renderer::export::ExportError;
pub use start::start;

use std::sync::Arc;

use ipc_channel::ipc::IpcError;
use glutin::event_loop::EventLoopProxy;
use tokio::sync::Mutex;

use crate::ipc_protocol::{ServerConnection, ClientRequest};
use crate::renderer_client::ClientId;

use app::App;
use access_control::AccessControl;
use renderer::display_list::DisplayList;
use event_loop_notifier::{EventLoopNotifier, MainThreadAction};

/// Serves requests from the client forever
async fn serve(
    conn: ServerConnection,
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: EventLoopProxy<MainThreadAction>,
) {
    let conn = Arc::new(conn);
    let app_control = Arc::new(AccessControl::new(app).await);
    let event_loop = Arc::new(EventLoopNotifier::new(event_loop));

    loop {
        let (client_id, request) = match conn.recv().await {
            Ok(req) => req,
            // Client has disconnected completely, no purpose in continuing this loop
            Err(IpcError::Disconnected) => break,
            Err(err) => panic!("unable to receive request from IPC client: {:?}", err),
        };

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
    event_loop: Arc<EventLoopNotifier>,
    request: ClientRequest,
) {
    use ClientRequest::*;
    let res = match request {
        CreateTurtle => {
            handlers::create_turtle(&conn, client_id, &app_control, &event_loop).await
        },

        Export(path, format) => {
            handlers::export_drawings(&conn, client_id, &app_control, &display_list, &path, format).await
        },

        NextEvent => {
            todo!()
        },

        DrawingProp(prop) => {
            handlers::drawing_prop(&conn, client_id, &app_control, prop).await
        },
        SetDrawingProp(prop_value) => {
            handlers::set_drawing_prop(&app_control, &event_loop, prop_value).await
        },
        ResetDrawingProp(prop) => {
            handlers::reset_drawing_prop(&app_control, &event_loop, prop).await
        },

        TurtleProp(id, prop) => {
            handlers::turtle_prop(&conn, client_id, &app_control, id, prop).await
        },
        SetTurtleProp(id, prop_value) => {
            handlers::set_turtle_prop(&app_control, &display_list, &event_loop, id, prop_value).await
        },
        ResetTurtleProp(id, prop) => {
            handlers::reset_turtle_prop(&app_control, &display_list, &event_loop, id, prop).await
        },
        ResetTurtle(id) => {
            handlers::reset_turtle(&app_control, &display_list, &event_loop, id).await
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

        Clear(id) => match id {
            Some(id) => handlers::clear_turtle(&app_control, &display_list, &event_loop, id).await,
            None => handlers::clear(&app_control, &display_list, &event_loop).await,
        },
    };

    match res {
        Ok(()) => {},
        //TODO: Use the error to figure out how to proceed
        //TODO: Store a flag somewhere to stop all remaining requests from continuing
        //  Maybe app_control.get() can return a Result?
        Err(err) => todo!("{}", err),
    }
}
