use query::{Query, Response};
use clock;

use std::default::Default;

/// Encapsulates runtime support that is environment-specific.
///
/// A Runtime is what hosts the user-provided turtle control logic and executes the render
/// commands, state queries, etc.
pub trait Runtime: Default {
    /// An abstraction around the clock available in the implementation's specific environment.
    type Clock: clock::Clock;

    /// Perform any necessary one-time initialization.
    // TODO perhaps have this be called on a type that then results in the type that has
    // `send_query`. As it is, the type system does not enforce that you call `initialize()` first,
    // and it breaks badly if you don't.
    fn initialize();

    /// Sends a query to the rendering logic and automatically decides whether or not to wait
    /// for a response.
    ///
    /// If a query does not require a response, this function will return immediately after
    /// sending the query
    fn send_query(&mut self, query: Query) -> Option<Response>;

    /// Write to some form of logging (for environments where eprintln doesn't work, like wasm)
    fn debug_log(s: &str);
}
