use std::sync::Arc;

use tokio::sync::Mutex;

use crate::ipc_protocol::ServerConnection;

use super::app::App;
use super::renderer::display_list::DisplayList;
use super::test_event_loop_notifier::EventLoopNotifier;

pub async fn main(server_name: String) {
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

    let conn = ServerConnection::connect(server_name)
        .expect("unable to establish turtle server connection");
    super::serve(conn, app, display_list, event_loop_notifier).await;
}
