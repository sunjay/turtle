#[cfg(not(any(feature = "test", test)))]
compile_error!("This module should only be included when compiling tests");

use std::sync::mpsc;

use crate::query::{Query, Response};

/// A special "renderer process" specifically for tests. Simulates the renderer process by
/// providing all of the same functionality and reusing internal parts of the server. No actual
/// process or additional threads are spawned.
pub struct RendererProcess {
    app: crate::app::TurtleApp,
    events: (mpsc::Sender<crate::Event>, mpsc::Receiver<crate::Event>),
    drawing: (mpsc::Sender<crate::query::DrawingCommand>, mpsc::Receiver<crate::query::DrawingCommand>),
}

impl RendererProcess {
    pub fn new() -> Self {
        Self {
            app: crate::app::TurtleApp::new(),
            events: mpsc::channel(),
            drawing: mpsc::channel(),
        }
    }

    pub fn send_query(&mut self, query: Query) -> Option<Response> {
        crate::server::handle_query_for_test_use_only(query, &mut self.app, &self.events.1, &self.drawing.0)
            .expect("test bug: a query failed to be successful")
    }
}
