//! Since the `turtle` crate provides a synchronous API that dispatches to the async API, we need
//! a global runtime that runs behind the scenes to drive everything.

use std::future::Future;

use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

/// The global runtime, spawned in the background the first time it is used
///
/// If this is never used, it will never spawn a runtime.
static RUNTIME: OnceCell<Runtime> = OnceCell::new();

pub fn block_on<F: Future>(future: F) -> F::Output {
    let runtime = RUNTIME.get_or_init(|| Runtime::new()
        .expect("unable to spawn tokio runtime"));
    runtime.handle().block_on(future)
}
