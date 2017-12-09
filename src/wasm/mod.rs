use query::{Query, Response};
use render_strategy::RenderStrategy;

pub struct WasmRenderStrategy;

impl RenderStrategy for WasmRenderStrategy {
    fn initialize() {
        // no op
    }

    fn send_query(&mut self, query: Query) -> Option<Response> {
        unimplemented!()
    }
}

impl Default for WasmRenderStrategy {
    fn default() -> Self {
        WasmRenderStrategy {}
    }
}
