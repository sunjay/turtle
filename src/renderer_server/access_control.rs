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
//! at the same time. On the other hand, if a turtle is drawing a line and then the clear request
//! is sent from another thread, the turtle should be able to finish drawing that line before the
//! clear request is executed.
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
//! be impossible to lock the data in between request executions in order to do rendering. We would
//! need some kind of prioritized locking system that allows a lock from the renderer to override
//! any other lock requests.
//!
//! The code below uses a separate data structure to track when it is appropriate for a request to
//! begin attempting to lock the data. This ensures that only one request is ever actually in the
//! mutex queue. That means that the renderer is free to lock the data when it needs to. The data
//! will become available to the renderer when that request is done executing.
//!
//! Note that requests can take a non-instant amount of time to execute. (That is, a request can
//! `await` during its execution.) That means that any locks need to be held across await points so
//! that a request completely finishes executing before the next request is notified that the lock
//! is available.
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

use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, Barrier};
use futures_util::future::join_all;

use super::state::DrawingState;
use super::app::{App, TurtleDrawings, TurtleId};

#[derive(Debug, Clone)]
pub enum RequiredTurtles {
    // NOTE: using "One" or "Two" instead of something more general like "Some(Vec<TurtleId>)"
    // allows common cases to avoid doing a bunch of small heap allocations. This enum can be
    // extended as necessary depending on how many turtles are required in the future. If a truly
    // variable number of turtles is needed for a request, it is fine to add a
    // "Some(Vec<TurtieId>)" variant to this enum.

    /// Request access to one turtle
    One(TurtleId),

    /// Request access to two turtles
    ///
    /// Note that if the two IDs are the same, this will cause a deadlock.
    Two(TurtleId, TurtleId),

    /// Request access to all the turtles
    All,
}

#[derive(Default, Debug, Clone)]
pub struct RequiredData {
    /// If true, the drawing state will be locked and provided in the data
    pub drawing: bool,

    /// Requests access to none, some, or all of the turtles
    pub turtles: Option<RequiredTurtles>,
}

/// Provides access to turtles
///
/// Due to some limitations of the locking APIs, these are not provided as lock guards. Each turtle
/// must be locked before it can be used. It is guaranteed that no other call to `get()` through
/// `AccessControl` will have access to the same turtles, so the locking should be uncontended. The
/// only caveat is that the renderer may have locked all of the turtles, so the command may have to
/// wait for that operation to complete.
#[derive(Debug)]
enum Turtles {
    // NOTE: A similar note to the one on `RequiredTurtles` applies here too

    /// Access to a single turtle
    One(Arc<Mutex<TurtleDrawings>>),

    /// Access to two turtles
    Two(Arc<Mutex<TurtleDrawings>>, Arc<Mutex<TurtleDrawings>>),

    /// Access to all the turtles
    All(Vec<Arc<Mutex<TurtleDrawings>>>),
}

#[derive(Debug)]
pub enum TurtlesGuard<'a> {
    // NOTE: A similar note to the one on `RequiredTurtles` applies here too

    /// Access to a single turtle
    One(MutexGuard<'a, TurtleDrawings>),

    /// Access to two turtles
    Two(MutexGuard<'a, TurtleDrawings>, MutexGuard<'a, TurtleDrawings>),

    /// Access to all the turtles
    All(Vec<MutexGuard<'a, TurtleDrawings>>),
}

/// A locked version of all the required data once it is ready
#[derive(Debug)]
pub struct DataGuard<'a> {
    /// If `RequiredData::drawing` was true, this field will contain the locked drawing state
    drawing: Option<MutexGuard<'a, DrawingState>>,

    /// The turtles requested in `RequiredData::turtles`
    turtles: Option<Turtles>,
}

impl<'a> DataGuard<'a> {
    /// Gets a mutable reference to the drawing state or panics if it was not requested in `get()`
    pub fn drawing_mut(&mut self) -> &mut DrawingState {
        self.drawing.as_mut()
            .expect("attempt to fetch drawing when it was not requested")
    }

    /// Gets the mutable locked turtles that were requested or panics if none were requested
    pub async fn turtles_mut(&mut self) -> TurtlesGuard<'_> {
        let turtles = self.turtles.as_mut()
            .expect("attempt to fetch turtles when none were requested");

        use Turtles::*;
        match turtles {
            One(turtle) => TurtlesGuard::One(turtle.lock().await),
            Two(turtle1, turtle2) => TurtlesGuard::Two(turtle1.lock().await, turtle2.lock().await),
            All(turtles) => TurtlesGuard::All(join_all(turtles.iter().map(|t| t.lock())).await),
        }
    }
}

/// Manages access to the app state, enforcing the rules around sequential consistency and
/// concurrent access
#[derive(Debug)]
pub struct AccessControl {
    app: Arc<App>,
}

impl AccessControl {
    pub fn new(app: Arc<App>) -> Self {
        Self {
            app,
        }
    }

    /// Adds a new turtle to the application state
    ///
    /// This does not need any ordering protection because it is impossible for any command to
    /// depend on the data for a turtle that hasn't been created yet.
    pub async fn add_turtle(&self) -> TurtleId {
        self.app.add_turtle().await
    }

    /// Requests the opportunity to potentially read or modify all turtles
    pub async fn get(&self, req_data: RequiredData) -> DataGuard<'_> {
        todo!()
    }
}
