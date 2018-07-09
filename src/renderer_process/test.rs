#[cfg(not(any(feature = "test", test)))]
compile_error!("This module should only be included when compiling tests");

use std::sync::mpsc;

use query::{Query, Response};

/// A special "renderer process" specifically for tests. Simulates the renderer process by
/// providing all of the same functionality and reusing internal parts of the server. No actual
/// process or additional threads are spawned.
pub struct RendererProcess {
    app: ::app::TurtleApp,
    events: (mpsc::Sender<::Event>, mpsc::Receiver<::Event>),
    drawing: (mpsc::Sender<::query::DrawingCommand>, mpsc::Receiver<::query::DrawingCommand>),
}

impl RendererProcess {
    pub fn new() -> Self {
        Self {
            app: ::app::TurtleApp::new(),
            events: mpsc::channel(),
            drawing: mpsc::channel(),
        }
    }

    pub fn send_query(&mut self, query: Query) -> Option<Response> {
        ::server::handle_query_for_test_use_only(query, &mut self.app, &self.events.1, &self.drawing.0)
            .expect("test bug: a query failed to be successful")
    }
}
