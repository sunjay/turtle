use std::ops::{Deref, DerefMut};

use turtle::random;

use grid::Grid;

#[derive(Debug)]
pub struct Maze {
    // The cells of the maze are stored row-wise
    // That means that each array in the outer array is a row of the maze
    grid: Grid,
}

impl Maze {
    /// Generates a new random maze
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
        }
    }

}

impl Deref for Maze {
    type Target = Grid;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for Maze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}
