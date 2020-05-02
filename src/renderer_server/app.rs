use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::{Mutex, MutexGuard};
use sharded_slab::Slab;

use super::state::{TurtleState, DrawingState};
use super::renderer::display_list::PrimHandle;

/// Slab uses `usize` for its ID type
type SlabId = usize;

/// The unique ID of a particular turtle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TurtleId(usize);

#[derive(Default, Debug)]
pub struct TurtleDrawings {
    pub state: TurtleState,
    pub drawings: Vec<PrimHandle>,
}

/// The entire state of the application, shared between threads in the server
#[derive(Default, Debug)]
pub struct App {
    /// The current state of the drawing
    drawing: Mutex<DrawingState>,
    /// The data for each turtle
    ///
    /// This field could be replaced with an append-only lock-free list or even an
    /// `RwLock<Vec<Mutex<T>>>` if it were possible to hold both the read guard and the inner mutex
    /// guard at the same time in the same struct. The main requirements here are that you can
    /// access multiple turtles concurrently (so it must be Sync) and that you can push new turtles
    /// in as needed. So lock-freedom isn't strictly necessary (see RwLock<Vec<Mutex<T>>>), but it
    /// is nice to have because it works well in an async-context (no blocking). The `appendlist`
    /// crate fits these requirements, but it is not Sync, so it can't be used here.
    turtles: Slab<Mutex<TurtleDrawings>>,
    /// Each `TurtleId` indexes into this field to get the ID used to access the slab
    turtle_ids: Mutex<Vec<SlabId>>,
}

impl App {
    /// Adds a new turtle to the application state, returning its `TurtleId`
    pub async fn add_turtle(&self) -> TurtleId {
        let slab_ids = self.turtle_ids.lock().await;
        let id = TurtleId(slab_ids.len());
        let slab_id = self.turtles.insert(Default::default())
            .expect("bug: unable to create turtle, shard is full");
        slab_ids.push(slab_id);
        id
    }

    /// Returns a mutable handle to the drawing state
    pub async fn drawing_mut(&self) -> MutexGuard<'_, DrawingState> {
        self.drawing.lock().await
    }

    /// Returns a handle to a the state and drawings of the given turtle
    pub async fn turtle(&self, id: TurtleId) -> Arc<Mutex<TurtleDrawings>> {
        let TurtleId(index) = id;
        let slab_id = self.turtle_ids.lock()[index];
        let turtles = self.turtles.read().await;
        turtles[index].clone()
    }
}
