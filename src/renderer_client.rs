/// Manages a single connection to a renderer server.
///
/// Each new client spawns a new server (i.e. a new window on desktop platforms).
///
/// This client can be cloned to create multiple connections to the same server.
#[derive(Debug)]
pub struct RendererClient {
}

impl Clone for RendererClient {
    fn clone(&self) -> Self {
        todo!()
    }
}
