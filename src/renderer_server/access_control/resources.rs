use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::collections::{VecDeque, HashMap};

use tokio::sync::oneshot;

use super::super::app::TurtleId;

// Using a synchronous Mutex since we don't need to keep it locked across await points
#[derive(Debug)]
pub struct DataReadyNotifier(Mutex<Option<oneshot::Sender<()>>>);

impl DataReadyNotifier {
    pub fn new(sender: oneshot::Sender<()>) -> Self {
        DataReadyNotifier(Mutex::new(Some(sender)))
    }

    /// Signals that the data is ready
    ///
    /// This method will panic if called more than once
    pub fn signal_ready(&self) {
        let DataReadyNotifier(sender) = self;

        // There are some cases (e.g. a panic) where AccessControl can get dropped before all of the
        // DataReadyNotifiers. In that case, the lock or send might fail and we can just ignore it.
        if let Ok(mut sender) = sender.lock() {
            let sender = sender.take()
                .expect("bug: only the last resource should notify that the data is ready");
            sender.send(()).unwrap_or(())
        }
    }
}

#[derive(Debug)]
pub struct PendingDataRequest {
    /// The number of resources currently being waited on
    ///
    /// Once this counter hits zero, the data request has been fulfilled and all required resources
    /// are available.
    needed: AtomicUsize,

    /// Used to signal the task waiting for data that the data is ready
    ///
    /// Only a the last resource being waited on will use this field.
    data_ready: DataReadyNotifier,
}

impl PendingDataRequest {
    /// Creates a new pending data request that is waiting for a single resource
    pub fn new(sender: oneshot::Sender<()>) -> Self {
        Self {
            needed: AtomicUsize::new(1),
            data_ready: DataReadyNotifier::new(sender),
        }
    }

    /// Records that another resource is necessary for this request to be fulfilled
    ///
    /// Note: this method should not be called after a data request has finished polling resources.
    pub fn add_needed_resource(&self) {
        self.needed.fetch_add(1, Ordering::SeqCst);
    }

    /// Records that one of the resources pending for this request is now available
    ///
    /// Decrements the number of resources being waited for and signals that all of the data is
    /// ready if this was the last resource being waited on.
    pub fn resource_ready(&self) {
        let prev_counter = self.needed.fetch_sub(1, Ordering::SeqCst);
        debug_assert_ne!(prev_counter, 0, "bug: inconsistent counter for resource (overflow)");

        let counter = prev_counter - 1;
        if counter == 0 {
            self.data_ready.signal_ready();
        }
    }
}

#[derive(Debug)]
pub struct Resource {
    /// If false, the resource is currently being held and any requests must be queued
    is_available: bool,
    /// The pending data requests that could not be fulfilled because this resource was unavailable
    pending: VecDeque<Arc<PendingDataRequest>>,
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            // All resources start out available since they have not been used yet
            is_available: true,
            pending: Default::default(),
        }
    }
}

impl Resource {
    pub fn is_available(&self) -> bool {
        self.is_available
    }

    pub fn push_data_request(&mut self, req: Arc<PendingDataRequest>) {
        self.pending.push_back(req);
    }

    /// Reserves the resource, thus making it unavailable until it is freed
    pub fn reserve(&mut self) {
        debug_assert!(self.is_available, "bug: attempt to reserve resource that is currently unavailable");

        self.is_available = false;
    }

    /// Frees this resource to the next pending request or marks it as available if no further
    /// requests are waiting to be processed
    ///
    /// This method should only be used once the resource is no longer available to the previous
    /// requestor
    pub fn free(&mut self) {
        debug_assert!(!self.is_available, "bug: attempt to free resource that is already available");

        match self.pending.pop_front() {
            Some(req) => {
                req.resource_ready();

                // Another request has claimed this resource, so it remains unavailable
            },

            None => {
                // No other request needs this resource, so it is now available
                self.is_available = true;
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct Resources {
    /// Controls access to the drawing
    drawing: Resource,
    /// Controls access to the turtles
    turtles: HashMap<TurtleId, Resource>,
}

impl Resources {
    pub fn drawing(&mut self) -> &mut Resource {
        &mut self.drawing
    }

    pub fn turtle(&mut self, id: TurtleId) -> &mut Resource {
        self.turtles.entry(id).or_default()
    }
}
