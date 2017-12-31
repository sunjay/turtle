use std::time;

use query::{Query, Response};
use runtime::Runtime;
use extensions::AsMillis;
use clock::{Clock, Timestamp};

use self::renderer_process::RendererProcess;

mod renderer_process;
mod server;
mod messenger;

/// A runtime for desktop OSs.
///
/// Spawns a separate process to render and keep track of turtle state so that the turtle logic can
/// take over the main thread of the original process.
pub struct DesktopRuntime {
    renderer_process: RendererProcess
}

impl DesktopRuntime {
    pub(crate) fn new() -> DesktopRuntime {
        server::start();
        DesktopRuntime {
            renderer_process: RendererProcess::new()
        }
    }
}

impl Runtime for DesktopRuntime {
    type Clock = SystemClock;
    type Rng = ::rand::ThreadRng;

    fn send_query(&mut self, query: Query) -> Option<Response> {
        self.renderer_process.send_query(query)
    }

    fn rng() -> Self::Rng {
        ::rand::thread_rng()
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
