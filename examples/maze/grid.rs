use std::ops::{Index, IndexMut};

use crate::cell::Cell;
use crate::wall::Wall;

const GRID_SIZE: usize = 16;

/// A row or column of cells
pub type Cells = [Cell; GRID_SIZE];

/// Used to determine whether to iterate over rows or columns
#[derive(Clone, Copy)]
enum GridIterTarget {
    Rows,
    Columns,
}

pub struct GridIter<'a> {
    grid: &'a [Cells; GRID_SIZE],
    target: GridIterTarget,
    current: usize,
    end: usize,
}

impl<'a> GridIter<'a> {
    fn new(grid: &'a [Cells; GRID_SIZE], target: GridIterTarget) -> Self {
        Self {
            grid,
            target,
            current: 0,
            end: match target {
                GridIterTarget::Rows => grid.len(),
                GridIterTarget::Columns => grid[0].len(),
            },
        }
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = GridCellIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let iter = GridCellIter::new(self.grid, self.target, self.current);

        self.current += 1;
        Some(iter)
    }
}

impl<'a> DoubleEndedIterator for GridIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let iter = GridCellIter::new(self.grid, self.target, self.end - 1);

        self.end -= 1;
        Some(iter)
    }
}

pub struct GridCellIter<'a> {
    grid: &'a [Cells; GRID_SIZE],
    target: GridIterTarget,
    /// The row or column being iterated over
    index: usize,
    /// The current position in the row or column being iterated over
    current: usize,
    /// Index of the item just after the last item in the row or column being iterated over
    end: usize,
}

impl<'a> GridCellIter<'a> {
    fn new(grid: &'a [Cells; GRID_SIZE], target: GridIterTarget, index: usize) -> Self {
        Self {
            grid,
            target,
            index,
            current: 0,
            end: match target {
                GridIterTarget::Rows => grid[0].len(),
                GridIterTarget::Columns => grid.len(),
            },
        }
    }

    fn get(&self, current: usize) -> &'a Cell {
        match self.target {
            GridIterTarget::Rows => &self.grid[self.index][current],
            GridIterTarget::Columns => &self.grid[current][self.index],
        }
    }
}

impl<'a> Iterator for GridCellIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let cell = self.get(self.current);

        self.current += 1;
        Some(cell)
    }
}

impl<'a> DoubleEndedIterator for GridCellIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let cell = self.get(self.end - 1);

        self.end -= 1;
        Some(cell)
    }
}

#[derive(Debug)]
pub struct Grid([Cells; GRID_SIZE]);

impl Index<usize> for Grid {
    type Output = Cells;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid([Cells::default(); GRID_SIZE])
    }

    /// Returns true if there is NO wall between two adjacent cells
    pub fn is_open_between(
        &self,
        (row1, col1): (usize, usize),
        (row2, col2): (usize, usize),
    ) -> bool {
        match (row2 as isize - row1 as isize, col2 as isize - col1 as isize) {
            // second position is north of first position
            (-1, 0) => {
                assert_eq!(self[row1][col1].north, self[row2][col2].south);
                self[row1][col1].north == Wall::Open
            }
            // second position is east of first position
            (0, 1) => {
                assert_eq!(self[row1][col1].east, self[row2][col2].west);
                self[row1][col1].east == Wall::Open
            }
            // second position is south of first position
            (1, 0) => {
                assert_eq!(self[row1][col1].south, self[row2][col2].north);
                self[row1][col1].south == Wall::Open
            }
            // second position is west of first position
            (0, -1) => {
                assert_eq!(self[row1][col1].west, self[row2][col2].east);
                self[row1][col1].west == Wall::Open
            }
            _ => unreachable!("Cells were not adjacent"),
        }
    }

    /// Removes the wall between two adjacent cells
    pub fn open_between(&mut self, (row1, col1): (usize, usize), (row2, col2): (usize, usize)) {
        match (row2 as isize - row1 as isize, col2 as isize - col1 as isize) {
            (-1, 0) => {
                self[row1][col1].north = Wall::Open;
                self[row2][col2].south = Wall::Open;
            }
            (0, 1) => {
                self[row1][col1].east = Wall::Open;
                self[row2][col2].west = Wall::Open;
            }
            (1, 0) => {
                self[row1][col1].south = Wall::Open;
                self[row2][col2].north = Wall::Open;
            }
            (0, -1) => {
                self[row1][col1].west = Wall::Open;
                self[row2][col2].east = Wall::Open;
            }
            _ => unreachable!("Cells were not adjacent"),
        }
    }

    /// Returns the cell positions adjacent in the four cardinal directions to the given cell
    /// position. Only returns valid cell positions.
    pub fn adjacent_cells(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        if row > 0 {
            // north
            positions.push((row - 1, col));
        }
        if col < self.row_size() - 1 {
            // east
            positions.push((row, col + 1));
        }
        if row < self.col_size() - 1 {
            // south
            positions.push((row + 1, col));
        }
        if col > 0 {
            // west
            positions.push((row, col - 1));
        }
        positions
    }

    /// Gets the cell at the given position
    pub fn get(&self, (row, col): (usize, usize)) -> &Cell {
        &self[row][col]
    }

    /// Returns the size of each row
    pub fn row_size(&self) -> usize {
        self.0[0].len()
    }

    /// Returns the size of each column
    pub fn col_size(&self) -> usize {
        self.0.len()
    }

    /// Returns the first row
    pub fn first_row(&self) -> GridCellIter<'_> {
        self.rows().next().unwrap()
    }

    /// Returns the last column
    pub fn last_col(&self) -> GridCellIter<'_> {
        self.cols().next_back().unwrap()
    }

    /// Returns an iterator over each row
    pub fn rows(&self) -> GridIter<'_> {
        GridIter::new(&self.0, GridIterTarget::Rows)
    }

    /// Returns an iterator over each row
    pub fn cols(&self) -> GridIter<'_> {
        GridIter::new(&self.0, GridIterTarget::Columns)
    }
}
