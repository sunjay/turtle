use query::{Query, Response};
use render_strategy::RenderStrategy;

pub struct CanvasRenderStrategy;

impl RenderStrategy for CanvasRenderStrategy {
    fn initialize() {
        // no op
    }

    fn send_query(&mut self, query: Query) -> Option<Response> {
        unimplemented!()
    }
}

impl Default for CanvasRenderStrategy {
    fn default() -> Self {
        CanvasRenderStrategy {}
    }
}
