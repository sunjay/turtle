use cell::Cell;

const GRID_SIZE: usize = 8;

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

        let iter = GridCellIter::new(
            self.grid,
            self.target,
            self.current
        );

        self.current += 1;
        Some(iter)
    }
}

impl<'a> DoubleEndedIterator for GridIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let iter = GridCellIter::new(
            self.grid,
            self.target,
            self.end - 1
        );

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
    // Index of the item just after the last item in the row or column being iterated over
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
            GridIterTarget::Rows => {
                &self.grid[self.index][current]
            },
            GridIterTarget::Columns => {
                &self.grid[current][self.index]
            },
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

impl Grid {
    pub fn new() -> Grid {
        Grid([Cells::default(); GRID_SIZE])
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
    pub fn first_row(&self) -> GridCellIter {
        self.rows().next().unwrap()
    }

    /// Returns the last row
    pub fn last_row(&self) -> GridCellIter {
        self.rows().next_back().unwrap()
    }

    /// Returns the first column
    pub fn first_col(&self) -> GridCellIter {
        self.cols().next().unwrap()
    }

    /// Returns the last column
    pub fn last_col(&self) -> GridCellIter {
        self.cols().next_back().unwrap()
    }

    /// Returns an iterator over each row
    pub fn rows(&self) -> GridIter {
        GridIter::new(&self.0, GridIterTarget::Rows)
    }

    /// Returns an iterator over each row
    pub fn cols(&self) -> GridIter {
        GridIter::new(&self.0, GridIterTarget::Columns)
    }
}
