use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use state::{TurtleState, DrawingState, Path};

/// Types that will be shared with another thread
pub type Shared<T> = Arc<RwLock<T>>;
/// Alias to help make the types more understandable without exposing as many implementation details
pub type ReadOnlyRef<'a, T> = RwLockReadGuard<'a, T>;
pub type MutableRef<'a, T> = RwLockWriteGuard<'a, T>;

/// A structure that provides read-only access to shared state
pub struct ReadOnly {
    turtle: Shared<TurtleState>,
    drawing: Shared<DrawingState>,
    /// A temporary path for use during animations
    temporary_path: Shared<Option<Path>>,
}

impl ReadOnly {
    pub fn turtle(&self) -> ReadOnlyRef<TurtleState> {
        self.turtle.read().expect("bug: Lock was poisoned")
    }

    pub fn drawing(&self) -> ReadOnlyRef<DrawingState> {
        self.drawing.read().expect("bug: Lock was poisoned")
    }

    pub fn temporary_path(&self) -> ReadOnlyRef<Option<Path>> {
        self.temporary_path.read().expect("bug: Lock was poisoned")
    }
}

/// Container for all the state of a turtle application
pub struct TurtleApp {
    turtle: Shared<TurtleState>,
    drawing: Shared<DrawingState>,
    /// A temporary path for use during animations
    temporary_path: Shared<Option<Path>>,
}

impl TurtleApp {
    pub fn new() -> Self {
        Self {
            turtle: Arc::new(RwLock::new(TurtleState::default())),
            drawing: Arc::new(RwLock::new(DrawingState::default())),
            temporary_path: Arc::new(RwLock::new(None)),
        }
    }

    /// Provide a read-only version of the state
    pub fn read_only(&self) -> ReadOnly {
        ReadOnly {
            turtle: Arc::clone(&self.turtle),
            drawing: Arc::clone(&self.drawing),
            temporary_path: Arc::clone(&self.temporary_path),
        }
    }

    /// Provides read-only access to the turtle state
    pub fn turtle(&self) -> ReadOnlyRef<TurtleState> {
        self.turtle.read().expect("bug: Lock was poisoned")
    }

    /// Provides mutable access to the turtle state
    pub fn turtle_mut(&mut self) -> MutableRef<TurtleState> {
        self.turtle.write().expect("bug: Lock was poisoned")
    }

    /// Provides read-only access to the drawing
    pub fn drawing(&self) -> ReadOnlyRef<DrawingState> {
        self.drawing.read().expect("bug: Lock was poisoned")
    }

    /// Provides mutable access to the drawing
    pub fn drawing_mut(&mut self) -> MutableRef<DrawingState> {
        self.drawing.write().expect("bug: Lock was poisoned")
    }

    /// Provides read-only access to the temporary path
    pub fn temporary_path(&self) -> ReadOnlyRef<Option<Path>> {
        self.temporary_path.read().expect("bug: Lock was poisoned")
    }

    /// Set the temporary_path to a new value, overwriting the previous one
    pub fn set_temporary_path(&mut self, path: Option<Path>) {
        let mut temp = self.temporary_path.write().expect("bug: Lock was poisoned");
        *temp = path;
    }

}
