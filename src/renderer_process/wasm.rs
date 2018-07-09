#[cfg(not(target_arch = "wasm32"))]
compile_error!("This module should only be included when compiling to wasm");

use query::{Query, Response};

/// A special "renderer process" specifically for communicating through the web assembly boundary
/// to the JavaScript that is running this program.
#[cfg(all(target_arch = "wasm32", not(any(feature = "test", test))))]
pub struct RendererProcess {
}

#[cfg(all(target_arch = "wasm32", not(any(feature = "test", test))))]
impl RendererProcess {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn send_query(&mut self, query: Query) -> Option<Response> {
        println!("{}", ::serde_json::to_string(&query).unwrap());
        None
    }
}
