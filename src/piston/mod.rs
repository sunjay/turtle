use query::{Query, Response};
use render_strategy::RenderStrategy;

use self::renderer_process::RendererProcess;

mod renderer;
mod renderer_process;
mod server;

pub struct PistonRenderStrategy {
    renderer_process: RendererProcess
}

impl PistonRenderStrategy {
    fn new() -> PistonRenderStrategy {
        PistonRenderStrategy {
            renderer_process: RendererProcess::new()
        }
    }
}

impl RenderStrategy for PistonRenderStrategy {
    fn initialize() {
        server::start()
    }

    fn send_query(&mut self, query: Query) -> Option<Response> {
        self.renderer_process.send_query(query)
    }
}

impl Default for PistonRenderStrategy {
    fn default() -> Self {
        PistonRenderStrategy::new()
    }
}
