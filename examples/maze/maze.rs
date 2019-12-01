use std::collections::{HashSet, VecDeque};

use turtle::rand::shuffle;

use crate::grid::Grid;

#[derive(Debug)]
pub struct Maze {
    /// The cells of the maze are stored row-wise
    /// That means that each array in the outer array is a row of the maze
    grid: Grid,
    start: (usize, usize),
    finish: (usize, usize),
}

impl Maze {
    /// Generates a new random maze
    pub fn generate() -> Self {
        let mut grid = Grid::new();

        let mut visited = HashSet::<(usize, usize)>::new();
        let mut remaining = VecDeque::<(usize, usize)>::new();
        remaining.push_back((0, 0));

        while let Some(current) = remaining.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            let adjacents = grid.adjacent_cells(current);

            if grid.get(current).is_all_closed() {
                // This cell hasn't been connected yet, let's try to do that
                let mut visited_adjacents = adjacents.iter().filter(|&p| visited.contains(p)).collect::<Vec<_>>();
                shuffle(&mut visited_adjacents);

                if let Some(&&adj) = visited_adjacents.first() {
                    grid.open_between(current, adj);
                }
            }

            let mut unvisited = adjacents.into_iter().filter(|p| !visited.contains(p)).collect::<Vec<_>>();

            if !unvisited.is_empty() {
                shuffle(&mut unvisited);

                let mut unvisited = unvisited.into_iter();
                // should exist because we just checked is_empty
                let next = unvisited.next().unwrap();
                remaining.push_front(next);
                remaining.extend(unvisited);

                grid.open_between(current, next);
            }
        }

        let start = (0, 0);
        let finish = (grid.col_size() - 1, grid.row_size() - 1);
        Self { grid, start, finish }
    }

    /// The start of the maze (row, col)
    pub fn start(&self) -> (usize, usize) {
        self.start
    }

    /// The exit of the maze (row, col)
    pub fn finish(&self) -> (usize, usize) {
        self.finish
    }

    /// Returns an immutable reference inner grid of the maze
    pub fn grid(&self) -> &Grid {
        &self.grid
    }
}
