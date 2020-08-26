//! Utility for managing concurrent access to the application state
//!
//! # Background
//!
//! Requests come in to the server in a certain order. Each request has a certain set of
//! "data dependencies". These dependencies are the parts of the state that the request wants to
//! read or modify. We want to avoid partially updating state or ending up in an inconsistent
//! state, so it is important that each request waits until **all** of the data it needs is
//! available before it operates on that data. Additionally, requests that do not depend on any of
//! the same data should be able to run concurrently.
//!
//! For example, if two turtles are drawing separate lines, they should be able to draw those lines
//! at the same time. On the other hand, if a turtle is drawing a line and then a clear request
//! is sent from another thread, the turtle should be able to finish drawing that line before the
//! clear request is executed. If in the meantime another request pertaining to a different turtle
//! comes in, that request should wait until the clear request has completed.
//!
//! More precisely, we are trying to enforce that:
//! * requests that depend on an overlapping subset of app data execute in the order in which they
//!   arrive in the request queue
//! * requests that are independent of each other (no shared data dependencies), may execute
//!   concurrently without having to wait for one another
//!
//! It is tough to do this with just a `Vec<Mutex<T>>` because while tokio's Mutex does guarantee
//! FIFO order, there is no way to put something back on the front of the mutex queue in the case
//! where *all* of its data dependencies aren't ready. Also, if we were to use a Mutex, it would
//! be impossible to lock the data in between request executions. That would mean that rendering
//! would be blocked until all requests have been processed. To deal with that we would need some
//! kind of prioritized locking system that allows a lock from the renderer to override any other
//! locks from requests.
//!
//! # Access Control
//!
//! The `AccessControl` data structure below is responsible for enforcing the properties outlined
//! above. Each piece of data shared by the renderer and the requests is stored behind a separate
//! mutex. To ensure that the renderer has priority over requests when locking the data, it does
//! not use the `AccessControl` type. Instead, it is given direct access to those mutexes and is
//! able to lock them whenever it needs to. Requests on the other hand are required to use the
//! `AccessControl` type to explicitly declare the data that they need. The `AccessControl` type
//! decides when to give each request permission to begin locking the data.
//!
//! Only a single request is ever given permission to begin locking a given set of mutexes.
//! Multiple requests with disjoint sets of data may be given permission at the same time to lock
//! their respective pieces of data. In all cases, the `AccessControl` type guarantees that only up
//! to one waiter will ever be in a mutex's queue or currently holding a mutex's lock. This is
//! important because it means that the renderer is guaranteed to always get priority access to the
//! data immediately after the current set of running requests. It can lock the data almost right
//! away without waiting for all queued requests to complete.
//!
//! Some requests do not complete "instantly" and may even have long await points (e.g. for an
//! animation delay). If not handled correctly, this can block rendering as a single request could
//! hold on to a lock for too long. To avoid this, the `AccessControl` type does not actually lock
//! all of the data as soon as a request is given permission to access it. Turtle data for example
//! is provided as the mutexes holding the data, not as mutex guards which would hold the locks.
//! This allows a request that needs long await points to unlock the data before those await points
//! begin. This enables the renderer to continue while still allowing requests to maintain access
//! over a longer period of time.
//!
//! The key here is that access to the data isn't bound by the amount of time the data is locked.
//! Instead, the `AccessControl` type provides the *opportunity* to lock, and waits for the request
//! to give up that opportunity before passing it on to the next request. Requests may unlock data
//! and still re-lock it later without worrying about losing their place to another request that
//! came afterwards.
//!
//! # Example
//!
//! Example: Suppose there are N = 4 turtles and you have the following requests:
//! - Request R1 depends on turtles: 1, 2, 3
//! - Request R2 depends on turtles: 4
//! - Request R3 depends on turtles: 3, 4
//! - Request R4 depends on turtles: 1, 2, 3, 4
//! - Request R5 depends on turtles: 1
//!
//! Expected behaviour:
//! 1. R1 and R2 execute concurrently, no shared dependencies
//! 2. R3 waits on both R1 and R2
//! 3. R4 waits on R3 (and implicitly R1 and R2)
//! 4. R5 waits on R4
//!
//! Conceptually, you can imagine that there is a queue for each turtle's data. The requests can
//! be sorted into those queues like so:
//!
//! 1: R1, R4, R5
//! 2: R1, R4
//! 3: R1, R3, R4
//! 4: R2, R3, R4
//!
//! The key here is that each request is listed in the order that it was in the original queue. A
//! request cannot execute until the request before it is done. A request can't be done until it's
//! at the front of all the queues it is in.

mod data_guard;
mod data_request;
mod resources;

pub use data_guard::*;
pub use data_request::*;
pub use resources::*;

use std::sync::Arc;

use tokio::sync::oneshot;
use parking_lot::Mutex;

use super::app::{App, TurtleId};

// Note: It is important that all of the resources be protected together by a Mutex rather than
// individually on a per-resource basis. This is how we guarantee that if a request needs multiple
// resources, it is able to reserve them all at once. Without that, two requests could race to
// reserve the same resources and end up deadlocked because the requests were queued in the wrong
// order.
type SharedResources = Arc<Mutex<Resources>>;

/// Manages access to the app state, enforcing the rules around sequential consistency and
/// concurrent access
#[derive(Debug)]
pub struct AccessControl {
    app: Arc<App>,
    resources: SharedResources,
}

impl AccessControl {
    /// Creates a struct that will manage access to the given App data
    ///
    /// This struct assumes that App will only be managed here. That is, App should never be
    /// mutated without first going through this struct. Reading data externally is fine.
    pub async fn new(app: Arc<App>) -> Self {
        assert_eq!(app.turtles_len().await, 0,
            "bug: access control assumes that turtles are only added through itself");

        let resources = SharedResources::default();

        Self {app, resources}
    }

    /// Adds a new turtle to the application state
    ///
    /// This does not need any ordering protection because no call to `get()` is allowed to depend
    /// on the data for a turtle that hasn't been created yet.
    pub async fn add_turtle(&self) -> TurtleId {
        let id = self.app.add_turtle().await;

        id
    }

    /// Requests the opportunity to potentially read or modify the drawing and all turtles
    ///
    /// See `get()` for more information
    pub async fn get_all(&self, data_req_queued: oneshot::Sender<()>) -> (DrawingData, Vec<TurtleData>) {
        // Record the IDs of the turtles that are currently available. This is necessary to
        // guarantee the soundness of requesting "all" of the turtles. No request is allowed to
        // depend on turtles that haven't been created yet, so "all" must be treated as all turtles
        // that currently exist when the request was sent. Only those turtles will be accessed, even
        // if more are added while the request waits.
        let turtles: Vec<_> = self.app.turtle_ids().await.collect();

        self.get((FetchDrawing, turtles), data_req_queued).await
    }

    /// Requests the opportunity to potentially read or modify the drawing
    ///
    /// See `get()` for more information
    pub async fn get_drawing(&self, data_req_queued: oneshot::Sender<()>) -> DrawingData {
        self.get(FetchDrawing, data_req_queued).await
    }

    /// Requests the opportunity to potentially read or modify some of the
    /// application state
    ///
    /// A message will be sent through `data_req_queued` when the data requests
    /// have been queued and the next call to get() may proceed.
    ///
    /// # Deadlock Warning
    ///
    /// **Warning:** If used incorrectly, this method can deadlock the program. For example, if the
    /// same resource is requested twice, one of the requests will be fulfilled, but the other will
    /// cause a deadlock since the same resource cannot be held twice.
    pub async fn get<R: DataRequest>(
        &self,
        data_req: R,
        data_req_queued: oneshot::Sender<()>,
    ) -> <R as DataRequest>::Output {
        // Request all the necessary data

        let mut pending_request: Option<(Arc<PendingDataRequest>, _)> = None;
        let generate_data_request = || match &pending_request {
            Some((req, _)) => {
                req.add_needed_resource();
                req.clone()
            },

            None => {
                let (sender, receiver) = oneshot::channel();
                let req = Arc::new(PendingDataRequest::new(sender));
                pending_request = Some((req.clone(), receiver));
                req
            },
        };

        {
            let mut res = self.resources.lock();
            data_req.poll_resources(&mut res, generate_data_request);
        }

        // Signal that all data requests have been queued and the next call to get() can proceed.
        // This is necessary since we can only guarantee request ordering if each request gets to
        // this point before the next call to `get`. Without this, we would have a race condition
        // between all callers of get() where the order would be randomly determined based on the
        // order that the tasks calling get() are scheduled.
        //
        // Ignoring errors since this just means that whoever was waiting to find out when the
        // requests have been queued no longer needs to know.
        data_req_queued.send(()).unwrap_or(());

        // Wait for any data that is not available yet
        if let Some((_, data_ready_receiver)) = pending_request {
            data_ready_receiver.await
                .expect("bug: tasks should notify about data being available before they are dropped");
        }

        // Fetch all the data that was requested (should only be contended by renderer)

        data_req.fetch_resources(&self.app, &self.resources).await
    }
}
