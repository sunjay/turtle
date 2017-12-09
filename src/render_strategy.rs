use query::{Query, Response};

use std::default::Default;

pub trait RenderStrategy: Default {
    /// Perform any necessary one-time initialization.
    // TODO perhaps have this be called on a type that then results in the type that has
    // `send_query`. As it is, the type system does not enforce that you call `initialize()` first,
    // and it breaks badly if you don't.
    fn initialize();

    /// Sends a query and automatically decides whether or not to wait for a response.
    ///
    /// If a query does not require a response, this function will return immediately after
    /// sending the query
    fn send_query(&mut self, query: Query) -> Option<Response>;
}
