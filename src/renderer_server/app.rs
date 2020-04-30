use serde::{Serialize, Deserialize};
use tokio::sync::{RwLock, Mutex};

use super::state::{TurtleState, DrawingState};

/// The unique ID of a particular turtle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TurtleId(usize);

#[derive(Debug)]
pub struct App {
    /// The current state of the drawing
    drawing: Mutex<DrawingState>,
    /// Each `TurtleId` indexes into this field
    turtles: RwLock<Vec<Mutex<TurtleState>>>,
}
