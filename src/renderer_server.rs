mod state;
mod app;
mod coords;
mod renderer;
mod backend;
mod handlers;
mod start;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "test", test))] {
        mod test_event_loop_notifier;
        use test_event_loop_notifier as event_loop_notifier;

    } else {
        mod event_loop_notifier;
        mod main;
    }
}

pub(crate) use app::TurtleId;
pub(crate) use backend::RendererServer;
pub use renderer::export::ExportError;
pub use start::start;

use ipc_channel::ipc::IpcError;
use tokio::sync::mpsc;
use parking_lot::{RwLock, Mutex};

use crate::ipc_protocol::{ServerSender, ServerOneshotSender, ServerReceiver, ClientRequest};
use crate::Event;

use app::{SharedApp, App};
use renderer::display_list::{SharedDisplayList, DisplayList};
use event_loop_notifier::EventLoopNotifier;

/// Serves requests from the client forever
async fn serve(
    conn: ServerSender,
    client_requests: ServerReceiver,
    app: SharedApp,
    display_list: SharedDisplayList,
    event_loop: EventLoopNotifier,
    mut events_receiver: mpsc::UnboundedReceiver<Event>,
    mut server_shutdown_receiver: mpsc::Receiver<()>,
) {
    loop {
        // This will either receive the next request or end this task
        let (client_id, request) = tokio::select! {
            // If the main thread shuts down successfully, this will receive Some(()). If the main
            // thread panics, this will return None. In either case, this loop needs to stop.
            _ = server_shutdown_receiver.recv() => break,

            req = client_requests.recv() => match req {
                Ok(req) => req,
                // Client has disconnected completely, no purpose in continuing this loop
                Err(IpcError::Disconnected) => break,
                Err(err) => panic!("unable to receive request from IPC client: {:?}", err),
            },
        };

        // Each request is executed immediately, in the order it arrives
        handle_handler_result(dispatch_request(
            ServerOneshotSender::new(client_id, conn.clone()),
            &app,
            &display_list,
            &event_loop,
            &mut events_receiver,
            request,
        ));

    }
}

fn dispatch_request(
    conn: ServerOneshotSender,
    app: &RwLock<App>,
    display_list: &Mutex<DisplayList>,
    event_loop: &EventLoopNotifier,
    events_receiver: &mut mpsc::UnboundedReceiver<Event>,
    request: ClientRequest,
) -> Result<(), handlers::HandlerError> {
    use ClientRequest::*;
    match request {
        CreateTurtle => {
            handlers::create_turtle(conn, &mut app.write(), event_loop)
        },

        Export(path, format) => {
            handlers::export_drawings(conn, &app.read(), &display_list.lock(), &path, format)
        },

        PollEvent => {
            handlers::poll_event(conn, events_receiver)
        },

        DrawingProp(prop) => {
            handlers::drawing_prop(conn, &app.read(), prop)
        },
        SetDrawingProp(prop_value) => {
            handlers::set_drawing_prop(&mut app.write(), event_loop, prop_value)
        },
        ResetDrawingProp(prop) => {
            handlers::reset_drawing_prop(&mut app.write(), event_loop, prop)
        },

        TurtleProp(id, prop) => {
            handlers::turtle_prop(conn, &app.read(), id, prop)
        },
        SetTurtleProp(id, prop_value) => {
            handlers::set_turtle_prop(&mut app.write(), &mut display_list.lock(), event_loop, id, prop_value)
        },
        ResetTurtleProp(id, prop) => {
            handlers::reset_turtle_prop(&mut app.write(), &mut display_list.lock(), event_loop, id, prop)
        },
        ResetTurtle(id) => {
            handlers::reset_turtle(&mut app.write(), &mut display_list.lock(), event_loop, id)
        },

        MoveForward(id, distance) => {
            handlers::move_forward(conn, &mut app.write(), &mut display_list.lock(), event_loop, id, distance)
        },
        MoveTo(id, target_pos) => {
            handlers::move_to(conn, &mut app.write(), &mut display_list.lock(), event_loop, id, target_pos)
        },
        RotateInPlace(id, angle, direction) => {
            handlers::rotate_in_place(conn, &mut app.write(), event_loop, id, angle, direction)
        },

        BeginFill(id) => {
            handlers::begin_fill(&mut app.write(), &mut display_list.lock(), event_loop, id)
        },
        EndFill(id) => {
            handlers::end_fill(&mut app.write(), id)
        },

        ClearAll => {
            handlers::clear_all(&mut app.write(), &mut display_list.lock(), event_loop)
        },
        ClearTurtle(id) => {
            handlers::clear_turtle(&mut app.write(), &mut display_list.lock(), event_loop, id)
        },

        DebugTurtle(id, angle_unit) => {
            handlers::debug_turtle(conn, &app.read(), id, angle_unit)
        },
        DebugDrawing => {
            handlers::debug_drawing(conn, &app.read())
        },
    }
}

fn handle_handler_result(res: Result<(), handlers::HandlerError>) {
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
