use std::sync::Arc;

use serde::{Serialize, Deserialize};
use parking_lot::RwLock;

use super::state::{TurtleState, DrawingState};
use super::renderer::display_list::PrimHandle;

/// The unique ID of a particular turtle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TurtleId(usize);

#[derive(Default, Debug)]
pub struct TurtleDrawings {
    /// The current state of this turtle: position, heading, etc.
    pub state: TurtleState,

    /// The drawings in the display list that have been created by this turtle
    pub drawings: Vec<PrimHandle>,

    /// If the turtle is currently filling, this is the handle to the fill polygon that new points
    /// should be appended to
    ///
    /// This handle will already be present in `drawings`, so it does not need to be added after
    /// the fill has begun.
    pub current_fill_polygon: Option<PrimHandle>,
}

/// The entire state of the application, shared between threads in the server
#[derive(Default, Debug)]
pub struct App {
    /// The current state of the drawing
    drawing: DrawingState,
    /// Each `TurtleId` indexes into this field
    ///
    /// Need to be very careful deleting from this field because the `TurtleId` returned from
    /// `add_turtle()` must remain unique and thus can never be repeated.
    turtles: Vec<TurtleDrawings>,
}

impl App {
    /// Adds a new turtle to the application state, returning its `TurtleId`
    pub fn add_turtle(&mut self) -> TurtleId {
        let id = TurtleId(self.turtles.len());
        self.turtles.push(Default::default());
        id
    }

    /// Returns a read-only handle to the drawing state
    pub fn drawing(&self) -> &DrawingState {
        &self.drawing
    }

    /// Returns a mutable handle to the drawing state
    pub fn drawing_mut(&mut self) -> &mut DrawingState {
        &mut self.drawing
    }

    /// Returns a read-only handle to the given turtle
    pub fn turtle(&self, id: TurtleId) -> &TurtleDrawings {
        let TurtleId(index) = id;
        &self.turtles[index]
    }

    /// Returns a mutable handle to the given turtle
    pub fn turtle_mut(&mut self, id: TurtleId) -> &mut TurtleDrawings {
        let TurtleId(index) = id;
        &mut self.turtles[index]
    }

    /// Returns an iterator over all of the turtles
    #[cfg_attr(feature = "test", allow(dead_code))] // Used in renderer, but not for tests
    pub fn turtles(&self) -> impl Iterator<Item=(TurtleId, &TurtleDrawings)> {
        (0..).zip(self.turtles.iter()).map(|(id, turtle)| (TurtleId(id), turtle))
    }

    /// Returns an iterator over all of the turtles
    pub fn turtles_mut(&mut self) -> impl Iterator<Item=(TurtleId, &mut TurtleDrawings)> {
        (0..).zip(self.turtles.iter_mut()).map(|(id, turtle)| (TurtleId(id), turtle))
    }
}

// Using `RwLock` so that requests that only need to read from the state can run concurrently with
// rendering.
pub type SharedApp = Arc<RwLock<App>>;
