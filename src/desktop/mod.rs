use std::time;

use query::{Query, Response};
use render_strategy::RenderStrategy;
use extensions::AsMillis;
use clock::{Clock, Timestamp};

use self::renderer_process::RendererProcess;

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
    type Clock = SystemClock;

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


/// A `Clock` backed by stdlib's time types.
pub struct SystemClock;

impl Clock for SystemClock {
    type Timestamp = time::Instant;

    fn now() -> Self::Timestamp {
        time::Instant::now()
    }
}

impl Timestamp for time::Instant {
    fn elapsed(&self) -> f64 {
        time::Instant::elapsed(self).as_millis() as f64
    }
}
