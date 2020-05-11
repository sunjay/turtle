use std::process;
use std::sync::Arc;

use tokio::{
    runtime::{Runtime, Handle},
    sync::Mutex,
};

use crate::ipc_protocol::ServerConnection;

use super::app::App;
use super::renderer::display_list::DisplayList;
use super::test_event_loop_notifier::EventLoopNotifier;

pub fn main() {
    // The runtime for driving async code
    let runtime = Runtime::new()
        .expect("unable to spawn tokio runtime to run turtle server process");

    // The state of the drawing and the state/drawings associated with each turtle
    let app = Arc::new(App::default());
    // All of the drawing primitives in the order in which they wil be drawn
    //
    // This is managed separately from the rest of the app state because the display list is shared
    // among pretty much everything and so critical sections containing the display list need to be
    // as short as possible.
    let display_list = Arc::new(Mutex::new(DisplayList::default()));

    // Create the proxy that will be given to the thread managing IPC
    let event_loop_notifier = Arc::new(EventLoopNotifier::new());

    let handle = runtime.handle().clone();
    spawn_async_server(handle, app, display_list, event_loop_notifier);

    // Simulates the fact that event_loop.run never exits
    process::exit(0);
}

fn spawn_async_server(
    handle: Handle,
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: Arc<EventLoopNotifier>,
) {
    // Spawn root task
    handle.spawn(async {
        let conn = ServerConnection::connect_stdin().await
            .expect("unable to establish turtle server connection");
        super::serve(conn, app, display_list, event_loop).await;
    });
}
