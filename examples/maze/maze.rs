use std::collections::{VecDeque, HashSet};
use std::ops::{Deref, DerefMut};

use turtle::{thread_rng, Rng};

use grid::Grid;

#[derive(Debug)]
pub struct Maze {
    // The cells of the maze are stored row-wise
    // That means that each array in the outer array is a row of the maze
    grid: Grid,
}

impl Maze {
    /// Generates a new random maze
    pub fn generate() -> Self {
        let mut grid = Grid::new();
        grid[0][0].mark_start();

        let mut visited = HashSet::<(usize, usize)>::new();
        let mut remaining = VecDeque::<(usize, usize)>::new();
        remaining.push_back((0, 0));

        let mut rng = thread_rng();
        while let Some(current) = remaining.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            let mut adjacents = grid.adjacent_cells(current)
                .into_iter()
                .filter(|p| !visited.contains(p))
                .collect::<Vec<_>>();

            if !adjacents.is_empty() {
                rng.shuffle(&mut adjacents);

                let mut unvisited = adjacents.into_iter();
                // should exist because we just checked is_empty
                let next = unvisited.next().unwrap();
                remaining.push_front(next);
                remaining.extend(unvisited);

                grid.open_between(current, next);
            }
        }

        Self {
            grid,
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
