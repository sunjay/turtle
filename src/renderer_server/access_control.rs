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
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use tokio::sync::{RwLock, Mutex, MutexGuard, Barrier, oneshot, mpsc};
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

impl RequiredTurtles {
    /// Returns the number of turtles required, up to the total number of turtles provided
    pub fn len(&self, turtles_len: usize) -> usize {
        use RequiredTurtles::*;
        match self {
            One(_) => 1,
            Two(_, _) => 2,
            All => turtles_len,
        }
    }
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
    One(SendDrop<Arc<Mutex<TurtleDrawings>>>),

    /// Access to two turtles
    Two(SendDrop<Arc<Mutex<TurtleDrawings>>>, SendDrop<Arc<Mutex<TurtleDrawings>>>),

    /// Access to all the turtles
    All(Vec<SendDrop<Arc<Mutex<TurtleDrawings>>>>),
}

/// Sends a message across a channel when dropping
#[derive(Debug)]
struct SendDrop<T> {
    value: T,
    sender: Option<oneshot::Sender<()>>,
}

impl<T> SendDrop<T> {
    fn new(value: T, sender: oneshot::Sender<()>) -> Self {
        Self {
            value,
            sender: Some(sender),
        }
    }
}

impl<T> Deref for SendDrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SendDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Drop for SendDrop<T> {
    fn drop(&mut self) {
        // unwrap() is safe because a struct cannot be dropped twice
        self.sender.take().unwrap().send(())
            .expect("bug: data channel should run forever");
    }
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

impl<'a> TurtlesGuard<'a> {
    pub fn one_mut(&mut self) -> &mut TurtleDrawings {
        use TurtlesGuard::*;
        match self {
            One(turtle) => turtle,
            _ => unreachable!("bug: expected exactly one turtle"),
        }
    }

    pub fn two_mut(&mut self) -> (&mut TurtleDrawings, &mut TurtleDrawings) {
        use TurtlesGuard::*;
        match self {
            Two(turtle1, turtle2) => (turtle1, turtle2),
            _ => unreachable!("bug: expected exactly two turtles"),
        }
    }

    pub fn all_mut(&mut self) -> &[MutexGuard<TurtleDrawings>] {
        use TurtlesGuard::*;
        match self {
            All(turtles) => turtles,
            _ => unreachable!("bug: expected all of the turtles"),
        }
    }
}

/// A locked version of all the required data once it is ready
#[derive(Debug)]
pub struct DataGuard<'a> {
    /// If `RequiredData::drawing` was true, this field will contain the locked drawing state
    drawing: Option<SendDrop<MutexGuard<'a, DrawingState>>>,

    /// The turtles requested in `RequiredData::turtles`
    turtles: Option<Turtles>,
}

impl<'a> DataGuard<'a> {
    /// Gets a mutable reference to the drawing state or panics if it was not requested in `get()`
    pub fn drawing_mut(&mut self) -> &mut DrawingState {
        self.drawing.as_mut()
            .expect("bug: attempt to fetch drawing when it was not requested")
    }

    /// Gets the mutable locked turtles that were requested or panics if none were requested
    pub async fn turtles_mut(&mut self) -> TurtlesGuard<'_> {
        let turtles = self.turtles.as_mut()
            .expect("bug: attempt to fetch turtles when none were requested");

        use Turtles::*;
        match turtles {
            One(turtle) => TurtlesGuard::One(turtle.lock().await),
            Two(turtle1, turtle2) => TurtlesGuard::Two(turtle1.lock().await, turtle2.lock().await),
            All(turtles) => TurtlesGuard::All(join_all(turtles.iter().map(|t| t.lock())).await),
        }
    }
}

/// Represents a request for access to a resource
///
/// Note that due to lifetime limitations, this does not manage sending the resource back to the
/// task waiting for access to it. It only acts to signal that the resource is ready to be
/// accessed. This proceeds in 4 stages:
#[derive(Debug)]
struct DataRequest {
    /// A barrier that all tasks will wait at before signaling that data is ready
    ///
    /// This ensures that all tasks send that signal simultaneously
    all_data_ready_barrier: Arc<Barrier>,

    /// This barrier must have the same size as the `data_ready` barrier and is used to ensure that
    /// access to all of the data ends at the same time. That way, even if one part of the data is
    /// freed up before the others, it will still appear as if all of them became available to the
    /// next task at the same time.
    all_complete_barrier: Arc<Barrier>,

    /// Allows the task to signal the waiting task that the data is ready
    ///
    /// The waiting task must send a message to the received channel when it is done with the
    /// resource it is using.
    data_ready: oneshot::Sender<oneshot::Sender<()>>,
}

/// Provides access to a task that manages access to a particular resource
#[derive(Debug)]
struct DataChannel {
    sender: mpsc::UnboundedSender<DataRequest>,
}

impl DataChannel {
    pub fn spawn() -> Self {
        let (sender, mut req_receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            loop {
                let DataRequest {
                    all_data_ready_barrier,
                    all_complete_barrier,
                    data_ready,
                } = match req_receiver.recv().await {
                    Some(req) => req,
                    // Main thread has quit, this task can quit too
                    None => break,
                };

                all_data_ready_barrier.wait().await;

                let (operation_complete, complete_receiver) = oneshot::channel();
                match data_ready.send(operation_complete) {
                    Ok(()) => {},
                    Err(_) => {}, // Could just be that a future was dropped
                };

                match complete_receiver.await {
                    Ok(()) => {},
                    Err(_) => {}, // Could just be that a future was dropped
                }

                // This barrier will eventually be reached no matter what
                all_complete_barrier.wait().await;
            }
        });

        Self {sender}
    }

    pub fn send(&self, req: DataRequest) {
        self.sender.send(req)
            .expect("bug: data channel should run forever")
    }
}

/// Manages access to the app state, enforcing the rules around sequential consistency and
/// concurrent access
#[derive(Debug)]
pub struct AccessControl {
    app: Arc<App>,
    drawing_channel: DataChannel,
    turtle_channels: RwLock<HashMap<TurtleId, DataChannel>>,
}

impl AccessControl {
    /// Creates a struct that will manage access to the given App data
    ///
    /// This struct assumes that App will only be managed here. That is, App should never be
    /// mutated without first going through this struct. Reading data externally is fine.
    pub async fn new(app: Arc<App>) -> Self {
        assert_eq!(app.turtles_len().await, 0,
            "bug: access control assumes that turtles are only added through itself");

        Self {
            app,
            drawing_channel: DataChannel::spawn(),
            turtle_channels: Default::default(),
        }
    }

    /// Adds a new turtle to the application state
    ///
    /// This does not need any ordering protection because it is impossible for any command to
    /// depend on the data for a turtle that hasn't been created yet.
    pub async fn add_turtle(&self) -> TurtleId {
        let id = self.app.add_turtle().await;

        let mut turtle_channels = self.turtle_channels.write().await;
        turtle_channels.insert(id, DataChannel::spawn());

        id
    }

    /// Requests the opportunity to potentially read or modify all turtles
    pub async fn get(&self, req_data: RequiredData) -> DataGuard<'_> {
        let RequiredData {drawing, turtles} = req_data;

        let turtles_len = self.app.turtles_len().await;
        let req_turtles = turtles.as_ref().map(|ts| ts.len(turtles_len)).unwrap_or(0);

        // Calculate the total number of data channels that we will request access from
        let total_reqs = if drawing { 1 } else { 0 } + req_turtles;

        let all_data_ready_barrier = Arc::new(Barrier::new(total_reqs));
        let all_complete_barrier = Arc::new(Barrier::new(total_reqs));

        // Start by sending data requests to the data channels for all requested data

        let drawing_ready = if drawing {
            let (data_ready, receiver) = oneshot::channel();
            self.drawing_channel.send(DataRequest {
                all_data_ready_barrier: all_data_ready_barrier.clone(),
                all_complete_barrier: all_complete_barrier.clone(),
                data_ready,
            });
            Some(receiver)
        } else {
            None
        };

        use RequiredTurtles::*;
        let mut turtles_ready = match &turtles {
            &Some(One(id)) => {
                let channels = self.turtle_channels.read().await;

                let (data_ready, receiver) = oneshot::channel();
                channels[&id].send(DataRequest {
                    all_data_ready_barrier: all_data_ready_barrier.clone(),
                    all_complete_barrier: all_complete_barrier.clone(),
                    data_ready,
                });

                vec![receiver]
            },

            &Some(Two(id1, id2)) => {
                let channels = self.turtle_channels.read().await;

                let (data_ready, receiver1) = oneshot::channel();
                channels[&id1].send(DataRequest {
                    all_data_ready_barrier: all_data_ready_barrier.clone(),
                    all_complete_barrier: all_complete_barrier.clone(),
                    data_ready,
                });

                let (data_ready, receiver2) = oneshot::channel();
                channels[&id2].send(DataRequest {
                    all_data_ready_barrier: all_data_ready_barrier.clone(),
                    all_complete_barrier: all_complete_barrier.clone(),
                    data_ready,
                });

                vec![receiver1, receiver2]
            },

            Some(All) => {
                let channels = self.turtle_channels.read().await;
                let mut receivers = Vec::with_capacity(turtles_len);

                for id in self.app.turtle_ids().await {
                    let (data_ready, receiver) = oneshot::channel();
                    channels[&id].send(DataRequest {
                        all_data_ready_barrier: all_data_ready_barrier.clone(),
                        all_complete_barrier: all_complete_barrier.clone(),
                        data_ready,
                    });
                    receivers.push(receiver);
                }

                receivers
            },

            None => Vec::new(),
        };

        // Now wait for data channels to signal that data is ready

        let drawing = match drawing_ready {
            Some(ready) => {
                // Wait to be signaled before locking
                let sender = ready.await
                    .expect("bug: data channel should run forever");
                Some(SendDrop::new(self.app.drawing_mut().await, sender))
            },
            None => None,
        };

        let turtles = match turtles {
            Some(One(id)) => {
                let sender = turtles_ready.remove(0).await
                    .expect("bug: data channel should run forever");
                debug_assert!(turtles_ready.is_empty());

                let turtle = self.app.turtle(id).await;
                Some(Turtles::One(SendDrop::new(turtle, sender)))
            },

            Some(Two(id1, id2)) => {
                let sender1 = turtles_ready.remove(0).await
                    .expect("bug: data channel should run forever");
                let sender2 = turtles_ready.remove(0).await
                    .expect("bug: data channel should run forever");
                debug_assert!(turtles_ready.is_empty());

                let turtle1 = self.app.turtle(id1).await;
                let turtle2 = self.app.turtle(id2).await;
                Some(Turtles::Two(
                    SendDrop::new(turtle1, sender1),
                    SendDrop::new(turtle2, sender2),
                ))
            },

            Some(All) => {
                let mut turtles = Vec::new();

                for (id, turtle_ready) in self.app.turtle_ids().await.zip(turtles_ready) {
                    let sender = turtle_ready.await
                        .expect("bug: data channel should run forever");

                    let turtle = self.app.turtle(id).await;
                    turtles.push(SendDrop::new(turtle, sender));
                }

                Some(Turtles::All(turtles))
            },

            None => None,
        };

        DataGuard {drawing, turtles}
    }
}
