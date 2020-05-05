//! Since the `turtle` crate provides a synchronous API that dispatches to the async API, we need
//! a global runtime that runs behind the scenes to drive everything.

use std::future::Future;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static! {
    /// The global runtime, spawned in the background the first time it is used
    static ref RUNTIME: Runtime = Runtime::new()
        .expect("unable to spawn tokio runtime");
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    RUNTIME.handle().block_on(future)
}
