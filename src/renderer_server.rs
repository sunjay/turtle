mod state;
mod app;
mod access_control;
mod coords;
mod renderer;
mod backend;
mod handlers;
#[cfg(not(any(feature = "test", test)))]
mod event_loop_notifier;
#[cfg(not(any(feature = "test", test)))]
mod main;
mod start;

#[cfg(any(feature = "test", test))]
mod test_event_loop_notifier;
#[cfg(any(feature = "test", test))]
use test_event_loop_notifier as event_loop_notifier;

pub(crate) use app::TurtleId;
pub(crate) use backend::RendererServer;
pub use renderer::export::ExportError;
pub use start::start;

use std::sync::Arc;

use ipc_channel::ipc::IpcError;
use tokio::sync::{mpsc, oneshot, Mutex};

use crate::ipc_protocol::{ServerConnection, ClientRequest};
use crate::renderer_client::ClientId;
use crate::Event;

use app::App;
use access_control::AccessControl;
use renderer::display_list::DisplayList;
use event_loop_notifier::EventLoopNotifier;

/// Serves requests from the client forever
async fn serve(
    conn: ServerConnection,
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: Arc<EventLoopNotifier>,
    events_receiver: mpsc::UnboundedReceiver<Event>,
    mut server_shutdown_receiver: mpsc::Receiver<()>,
) {
    let conn = Arc::new(conn);
    let app_control = Arc::new(AccessControl::new(app).await);
    let events_receiver = Arc::new(Mutex::new(events_receiver));

    loop {
        // This will either receive the next request or end this task
        let (client_id, request) = tokio::select! {
            // If the main thread shuts down successfully, this will receive Some(()).
            // If the main thread panics, this will return None. In either case, this task needs to
            // end immediately.
            _ = server_shutdown_receiver.recv() => break,

            req = conn.recv() => match req {
                Ok(req) => req,
                // Client has disconnected completely, no purpose in continuing this loop
                Err(IpcError::Disconnected) => break,
                Err(err) => panic!("unable to receive request from IPC client: {:?}", err),
            },
        };

        // To preserve the ordering of requests in cases where they can't run concurrently, we use
        // a channel to synchronize that each request
        let (data_req_queued, data_req_queued_receiver) = oneshot::channel();

        // Each incoming request is given its own task configured specifically for each kind of
        // request. Having separate tasks allows requests that can run in parallel to do so.
        tokio::spawn(run_request(
            data_req_queued,
            conn.clone(),
            client_id,
            app_control.clone(),
            display_list.clone(),
            event_loop.clone(),
            events_receiver.clone(),
            request,
        ));

        // Check if we are ready for the next request to be processed
        // Ignoring error because if data_req_queued was dropped, it probably just means that the
        // request didn't need to call AccessControl::get()
        data_req_queued_receiver.await.unwrap_or(());
    }
}

async fn run_request(
    data_req_queued: oneshot::Sender<()>,
    conn: Arc<ServerConnection>,
    client_id: ClientId,
    app_control: Arc<AccessControl>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: Arc<EventLoopNotifier>,
    events_receiver: Arc<Mutex<mpsc::UnboundedReceiver<Event>>>,
    request: ClientRequest,
) {
    use ClientRequest::*;
    let res = match request {
        CreateTurtle => {
            handlers::create_turtle(&conn, client_id, &app_control, &event_loop).await
        },

        Export(path, format) => {
            handlers::export_drawings(data_req_queued, &conn, client_id, &app_control, &display_list, &path, format).await
        },

        PollEvent => {
            // NOTE: Technically, because this does not send to `data_req_queued`, it is possible
            // to have several callers of `poll_event` race to get the next event. This appears to
            // be fine though because we don't guarantee the ordering of events if they are polled
            // from multiple threads/tasks. Having events follow the order of requests doesn't
            // really matter if strict ordering isn't necessary.
            handlers::poll_event(&conn, client_id, &events_receiver).await
        },

        DrawingProp(prop) => {
            handlers::drawing_prop(data_req_queued, &conn, client_id, &app_control, prop).await
        },
        SetDrawingProp(prop_value) => {
            handlers::set_drawing_prop(data_req_queued, &app_control, &event_loop, prop_value).await
        },
        ResetDrawingProp(prop) => {
            handlers::reset_drawing_prop(data_req_queued, &app_control, &event_loop, prop).await
        },

        TurtleProp(id, prop) => {
            handlers::turtle_prop(data_req_queued, &conn, client_id, &app_control, id, prop).await
        },
        SetTurtleProp(id, prop_value) => {
            handlers::set_turtle_prop(data_req_queued, &app_control, &display_list, &event_loop, id, prop_value).await
        },
        ResetTurtleProp(id, prop) => {
            handlers::reset_turtle_prop(data_req_queued, &app_control, &display_list, &event_loop, id, prop).await
        },
        ResetTurtle(id) => {
            handlers::reset_turtle(data_req_queued, &app_control, &display_list, &event_loop, id).await
        },

        MoveForward(id, distance) => {
            handlers::move_forward(data_req_queued, &conn, client_id, &app_control, &display_list, &event_loop, id, distance).await
        },
        MoveTo(id, target_pos) => {
            handlers::move_to(data_req_queued, &conn, client_id, &app_control, &display_list, &event_loop, id, target_pos).await
        },
        RotateInPlace(id, angle, direction) => {
            handlers::rotate_in_place(data_req_queued, &conn, client_id, &app_control, &event_loop, id, angle, direction).await
        },

        BeginFill(id) => {
            handlers::begin_fill(data_req_queued, &app_control, &display_list, &event_loop, id).await
        },
        EndFill(id) => {
            handlers::end_fill(data_req_queued, &app_control, id).await
        },

        Clear(id) => match id {
            Some(id) => handlers::clear_turtle(data_req_queued, &app_control, &display_list, &event_loop, id).await,
            None => handlers::clear(data_req_queued, &app_control, &display_list, &event_loop).await,
        },
    };

    use handlers::HandlerError::*;
    match res {
        Ok(()) => {},
        Err(IpcChannelError(err)) => panic!("Error while serializing response: {}", err),
        // Task managing window has ended, this task will end soon too.
        //TODO: This potentially leaves the turtle/drawing state in an inconsistent state. Should
        // we deal with that somehow? Panicking doesn't seem appropriate since this probably isn't
        // an error, but we should definitely stop processing commands and make sure the process
        // ends shortly after.
        Err(EventLoopClosed(_)) => {},
    }
}
