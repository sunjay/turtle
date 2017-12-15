use query::{Query, Response};
use clock;

/// Encapsulates runtime support that is environment-specific.
///
/// A Runtime is what hosts the user-provided turtle control logic and executes the render
/// commands, state queries, etc.
pub trait Runtime {
    /// An abstraction around the clock available in the implementation's specific environment.
    type Clock: clock::Clock;

    /// Sends a query to the rendering logic and automatically decides whether or not to wait
    /// for a response.
    ///
    /// If a query does not require a response, this function will return immediately after
    /// sending the query
    fn send_query(&mut self, query: Query) -> Option<Response>;

    /// Write to some form of logging (for environments where eprintln doesn't work, like wasm)
    fn debug_log(s: &str);
}
