use serde::{Serialize, Deserialize};
use tokio::sync::{RwLock, Mutex};

use super::state::{TurtleState, DrawingState};
use super::renderer::display_list::PrimHandle;

//TODO: pub type TurtleDrawingsGuard<'a> = MappedRwLockReadGuard<'a, MutexGuard<'a, TurtleDrawings>>;

/// The unique ID of a particular turtle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TurtleId(usize);

#[derive(Default, Debug)]
pub struct TurtleDrawings {
    pub state: TurtleState,
    pub drawings: Vec<PrimHandle>,
}

/// The entire state of the application
#[derive(Default, Debug)]
pub struct App {
    /// The current state of the drawing
    drawing: Mutex<DrawingState>,
    /// Each `TurtleId` indexes into this field
    ///
    /// Note that we have to be very careful deleting from this field since we don't want to
    /// invalidate any `TurtleId`.
    ///
    /// The outer `RwLock` makes it possible to push into the `Vec` using `write` and also lock
    /// multiple items in the `Vec` at the same time using `read`.
    turtles: RwLock<Vec<Mutex<TurtleDrawings>>>,
}

impl App {
    /// Adds a new turtle to the application state, returning its `TurtleId`
    pub async fn add_turtle(&self) -> TurtleId {
        let mut turtles = self.turtles.write().await;
        let id = TurtleId(turtles.len());
        turtles.push(Default::default());
        id
    }

    //TODO: Wait for https://github.com/tokio-rs/tokio/issues/2471
    ///// Returns a mutable handle to a the state and drawings of the given turtle
    //pub async fn turtle_mut(&self, id: TurtleId) -> TurtleDrawingsGuard {
    //    let TurtleId(index) = id;
    //    RwLockReadGuard::map(self.turtles.read().await, |turtles| turtles[index].lock()).await
    //}
}
