use query::{Query, Response};
use render_strategy::RenderStrategy;

use self::renderer_process::RendererProcess;

mod renderer;
mod renderer_process;
mod server;

pub struct DesktopRenderStrategy {
    renderer_process: RendererProcess
}

impl DesktopRenderStrategy {
    fn new() -> DesktopRenderStrategy {
        DesktopRenderStrategy {
            renderer_process: RendererProcess::new()
        }
    }
}

impl RenderStrategy for DesktopRenderStrategy {
    fn initialize() {
        server::start()
    }

    fn send_query(&mut self, query: Query) -> Option<Response> {
        self.renderer_process.send_query(query)
    }
}

impl Default for DesktopRenderStrategy {
    fn default() -> Self {
        DesktopRenderStrategy::new()
    }
}
