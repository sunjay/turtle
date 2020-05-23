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

use std::sync::Arc;
use std::collections::HashMap;

use tokio::sync::{RwLock, Mutex, MutexGuard, Barrier, oneshot, mpsc};
//TODO: Replace this with a `tokio` equivalent when tokio-rs/tokio#2478 is resolved:
//  https://github.com/tokio-rs/tokio/issues/2478
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
    #[allow(dead_code)] //TODO(#16): This will be used for the multiple turtles feature (for Turtle::clone())
    Two(TurtleId, TurtleId),

    /// Request access to all the turtles that exist at the current time
    ///
    /// Even if more turtles are added while the waiting for access, this will still only provide
    /// the turtles that existed when the `get()` call was first processed.
    All,
}

impl RequiredTurtles {
    /// Returns the number of turtles required, up to the provided total number of turtles
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
///
/// This "limitation" turns out to be convenient, as it allows turtle locks to be unlocked
/// temporarily during an animation. This unlocking is critical, as without it the renderer could
/// not draw while an animation was taking place.
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

impl<'a> TurtlesGuard<'a> {
    pub fn one_mut(&mut self) -> &mut TurtleDrawings {
        use TurtlesGuard::*;
        match self {
            One(turtle) => turtle,
            _ => unreachable!("bug: expected exactly one turtle"),
        }
    }

    #[allow(dead_code)] //TODO(#16): This will be used for the multiple turtles feature (for Turtle::clone())
    pub fn two_mut(&mut self) -> (&mut TurtleDrawings, &mut TurtleDrawings) {
        use TurtlesGuard::*;
        match self {
            Two(turtle1, turtle2) => (turtle1, turtle2),
            _ => unreachable!("bug: expected exactly two turtles"),
        }
    }

    pub fn all_mut(&mut self) -> &mut [MutexGuard<'a, TurtleDrawings>] {
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
    drawing: Option<MutexGuard<'a, DrawingState>>,

    /// The turtles requested in `RequiredData::turtles`
    turtles: Option<Turtles>,

    /// Channel to report to when the data guard is dropped
    operation_complete_sender: Option<oneshot::Sender<()>>,
}

impl<'a> Drop for DataGuard<'a> {
    fn drop(&mut self) {
        // unwrap() is safe because a struct cannot be dropped twice
        let sender = self.operation_complete_sender.take().unwrap();
        // There are some cases (e.g. a panic) where AccessControl can get dropped before
        // DataGuard. In that case, the send might fail and we can just ignore that. This should
        // never fail otherwise because the tasks managing access to data should run forever.
        sender.send(()).unwrap_or(())
    }
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
            Two(turtle1, turtle2) => {
                let (turtle1, turtle2) = tokio::join!(turtle1.lock(), turtle2.lock());
                TurtlesGuard::Two(turtle1, turtle2)
            },
            All(turtles) => TurtlesGuard::All(join_all(turtles.iter().map(|t| t.lock())).await),
        }
    }
}

/// Represents a request for access to a resource
///
/// Note that due to lifetime limitations, this does not manage sending the resource back to the
/// task waiting for access to it. It only acts to signal that the resource is ready to be
/// accessed.
#[derive(Debug)]
struct DataRequest {
    /// A barrier that all tasks will wait at before signaling that data is ready
    ///
    /// This ensures that all the requested data must be ready before proceeding to lock any of it
    all_data_ready_barrier: Arc<Barrier>,

    /// This barrier must have the same size as the `data_ready` barrier and is used to ensure that
    /// access to all of the data ends at the same time. This is necessary since only a single data
    /// channel (the leader) is ever aware that the data is no longer being accessed. That channel
    /// will be the last one to wait at this barrier and thus all of the data will become available
    /// again at the same time.
    all_complete_barrier: Arc<Barrier>,

    /// Used to signal the task waiting for data that the data is ready
    ///
    /// Only a single data channel (the leader) will use this field. That channel will then send a
    /// oneshot channel that will be used to indicate when the data is no longer going to be used.
    data_ready: Arc<Mutex<Option<oneshot::Sender<oneshot::Sender<()>>>>>,
}

/// Provides the ability to communicate with a task that manages access to a particular resource
#[derive(Debug)]
struct DataChannel {
    sender: mpsc::UnboundedSender<DataRequest>,
}

impl DataChannel {
    pub fn spawn() -> Self {
        let (sender, mut req_receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            // Run until recv() fails since that means that the main access control task has quit
            while let Some(req) = req_receiver.recv().await {
                let DataRequest {all_data_ready_barrier, all_complete_barrier, data_ready} = req;

                // Wait until all of the data is ready at the same time
                let res = all_data_ready_barrier.wait().await;

                if res.is_leader() {
                    // Notify that the data is ready
                    let data_ready = data_ready.lock().await.take()
                        .expect("only the leader should use the data ready channel");

                    let (operation_complete, complete_receiver) = oneshot::channel();

                    // Wait until the data has been dropped and will no longer be used
                    //
                    // Ignoring error because even if this fails, it just means that the future
                    // holding the data was dropped. In that case, the data is still now available
                    // again for the next task waiting for it.
                    data_ready.send(operation_complete).unwrap_or(());

                    // Even though only a single task is waiting on this channel, all of the tasks
                    // will wait on the barrier below until the data is no longer being used.
                    //
                    // Ignoring error because if this fails it could just be that the future
                    // waiting for the `operation_complete` channel was dropped. In that case, the
                    // data is still available again for the next task waiting for it.
                    complete_receiver.await.unwrap_or(());
                }

                // Wait for all of the data to stop being used before processing the next request
                //
                // This barrier must eventually be reached no matter what happens above. If all
                // data channels do not get to this point we can get into a situation where nothing
                // can make progress.
                all_complete_barrier.wait().await;
            }
        });

        Self {sender}
    }

    pub fn send(&self, req: DataRequest) {
        self.sender.send(req)
            .expect("bug: tasks managing access to data should run forever");
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
    /// This does not need any ordering protection because no call to `get()` is allowed to depend
    /// on the data for a turtle that hasn't been created yet.
    pub async fn add_turtle(&self) -> TurtleId {
        let id = self.app.add_turtle().await;

        let mut turtle_channels = self.turtle_channels.write().await;
        turtle_channels.insert(id, DataChannel::spawn());

        id
    }

    /// Requests the opportunity to potentially read or modify all turtles
    ///
    /// A message will be sent through `data_req_queued` when the data requests have been queued and
    /// the next call to get() may proceed.
    pub async fn get(
        &self,
        req_data: RequiredData,
        data_req_queued: oneshot::Sender<()>,
    ) -> DataGuard<'_> {
        let RequiredData {drawing, turtles} = req_data;

        //TODO: Explore if there is a different formulation of this struct that is simpler but
        // still accomplishes all of the same goals.

        let turtles_len = self.app.turtles_len().await;
        let req_turtles = turtles.as_ref().map(|ts| ts.len(turtles_len)).unwrap_or(0);

        // Calculate the total number of data tasks that we will request access from
        let total_reqs = if drawing { 1 } else { 0 } + req_turtles;

        // Send data requests to the tasks managing access to all the requested data

        let all_data_ready_barrier = Arc::new(Barrier::new(total_reqs));
        let all_complete_barrier = Arc::new(Barrier::new(total_reqs));
        let (data_ready, data_ready_receiver) = oneshot::channel();
        let data_ready = Arc::new(Mutex::new(Some(data_ready)));

        if drawing {
            self.drawing_channel.send(DataRequest {
                all_data_ready_barrier: all_data_ready_barrier.clone(),
                all_complete_barrier: all_complete_barrier.clone(),
                data_ready: data_ready.clone(),
            });
        }

        // Record the IDs that are currently available at the time that data is requested. This is
        // necessary to guarantee the soundness of `RequiredTurtles::All`. No request is allowed to
        // depend on turtles that haven't been created yet, so `All` must be treated as all turtles
        // that currently exist when the request was sent. Only those turtles will be accessed,
        // even if more are added while the request waits.
        let ids = self.app.turtle_ids().await;

        use RequiredTurtles::*;
        match &turtles {
            &Some(One(id)) => {
                let channels = self.turtle_channels.read().await;
                channels[&id].send(DataRequest {
                    all_data_ready_barrier: all_data_ready_barrier.clone(),
                    all_complete_barrier: all_complete_barrier.clone(),
                    data_ready: data_ready.clone(),
                });
            },

            &Some(Two(id1, id2)) => {
                let channels = self.turtle_channels.read().await;
                channels[&id1].send(DataRequest {
                    all_data_ready_barrier: all_data_ready_barrier.clone(),
                    all_complete_barrier: all_complete_barrier.clone(),
                    data_ready: data_ready.clone(),
                });

                channels[&id2].send(DataRequest {
                    all_data_ready_barrier: all_data_ready_barrier.clone(),
                    all_complete_barrier: all_complete_barrier.clone(),
                    data_ready: data_ready.clone(),
                });
            },

            Some(All) => {
                let channels = self.turtle_channels.read().await;
                for id in ids.clone() {
                    channels[&id].send(DataRequest {
                        all_data_ready_barrier: all_data_ready_barrier.clone(),
                        all_complete_barrier: all_complete_barrier.clone(),
                        data_ready: data_ready.clone(),
                    });
                }
            },

            None => {},
        }

        // Signal that all data requests have been queued and the next call to get() can proceed.
        // This is very important to get the ordering guarantees we are going for. Without this,
        // we would have a race condition between all callers of get() where the order would be
        // randomly determined based on the order the tasks calling get() are scheduled.
        // Ignoring errors since this just means that whoever was waiting to find out when the
        // requests have been queued no longer needs to know.
        data_req_queued.send(()).unwrap_or(());

        // Now wait for data ready channel to signal that data is ready
        let operation_complete_sender = data_ready_receiver.await
            .expect("bug: tasks managing access to data should run forever");

        // Lock all the data that was requested (should only be contended by renderer)

        // NOTE: To avoid deadlocking, all of the code follows a consistent locking order:
        //
        // 1. drawing.lock() (done below)
        // 2. turtles.lock() (done partially here and then in handler code)
        // 3. display_list.lock() (done in handler code as-needed)
        // 4. event_loop.lock() (done in handler code as-needed)
        //
        // Any of these steps may be omitted, but the order must always be consistent.

        let drawing = if drawing {
            let drawing = self.app.drawing_mut().await;
            Some(drawing)
        } else { None };

        let turtles = match turtles {
            Some(One(id)) => {
                let turtle = self.app.turtle(id).await;
                Some(Turtles::One(turtle))
            },

            Some(Two(id1, id2)) => {
                let (turtle1, turtle2) = tokio::join!(self.app.turtle(id1), self.app.turtle(id2));
                Some(Turtles::Two(turtle1, turtle2))
            },

            Some(All) => {
                let turtles = join_all(ids.map(|id| self.app.turtle(id))).await;
                Some(Turtles::All(turtles))
            },

            None => None,
        };

        let operation_complete_sender = Some(operation_complete_sender);
        DataGuard {drawing, turtles, operation_complete_sender}
    }
}
