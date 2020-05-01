mod renderer_server_process;

pub use renderer_server_process::*;

use serde::{Serialize, Deserialize};

/// A unique ID used to multiplex responses on the client side
///
/// Treated as an opaque value on the server that is returned back to the client with the response
/// to a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(usize);

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
