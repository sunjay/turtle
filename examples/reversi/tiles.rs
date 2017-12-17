use std::ops::{Index, IndexMut};

use board::Piece;

const SIZE: usize = 8;

/// (Row, Column)
pub type Position = (usize, usize);

/// A row or column of pieces
pub type Pieces = [Option<Piece>; SIZE];

/// Used to determine whether to iterate over rows or columns
#[derive(Clone, Copy)]
enum TilesIterTarget {
    Rows,
    Columns,
    DiagonalsTLBR,
    DiagonalsTRBL,
}

pub struct TilesIter<'a> {
    grid: &'a [Pieces; SIZE],
    target: TilesIterTarget,
    current: usize,
    end: usize,
}

impl<'a> TilesIter<'a> {
    fn new(grid: &'a [Pieces; SIZE], target: TilesIterTarget) -> Self {
        Self {
            grid,
            target,
            current: 0,
            end: match target {
                TilesIterTarget::Rows => grid.len(),
                TilesIterTarget::Columns => grid[0].len(),
                TilesIterTarget::DiagonalsTLBR => unimplemented!(),
                TilesIterTarget::DiagonalsTRBL => unimplemented!(),
            },
        }
    }
}

impl<'a> Iterator for TilesIter<'a> {
    type Item = TilesPieceIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let iter = TilesPieceIter::new(
            self.grid,
            self.target,
            self.current
        );

        self.current += 1;
        Some(iter)
    }
}

impl<'a> DoubleEndedIterator for TilesIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let iter = TilesPieceIter::new(
            self.grid,
            self.target,
            self.end - 1
        );

        self.end -= 1;
        Some(iter)
    }
}

pub struct TilesPieceIter<'a> {
    grid: &'a [Pieces; SIZE],
    target: TilesIterTarget,
    /// The row or column being iterated over
    index: usize,
    /// The current position in the row or column being iterated over
    current: usize,
    // Index of the item just after the last item in the row or column being iterated over
    end: usize,
}

impl<'a> TilesPieceIter<'a> {
    fn new(grid: &'a [Pieces; SIZE], target: TilesIterTarget, index: usize) -> Self {
        Self {
            grid,
            target,
            index,
            current: 0,
            end: match target {
                TilesIterTarget::Rows => grid[0].len(),
                TilesIterTarget::Columns => grid.len(),
                TilesIterTarget::DiagonalsTLBR => unimplemented!(),
                TilesIterTarget::DiagonalsTRBL => unimplemented!(),
            },
        }
    }

    fn get(&self, current: usize) -> &'a Option<Piece> {
        match self.target {
            TilesIterTarget::Rows => {
                &self.grid[self.index][current]
            },
            TilesIterTarget::Columns => {
                &self.grid[current][self.index]
            },
            TilesIterTarget::DiagonalsTLBR => unimplemented!(),
            TilesIterTarget::DiagonalsTRBL => unimplemented!(),
        }
    }
}

impl<'a> Iterator for TilesPieceIter<'a> {
    type Item = &'a Option<Piece>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let cell = self.get(self.current);

        self.current += 1;
        Some(cell)
    }
}

impl<'a> DoubleEndedIterator for TilesPieceIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let cell = self.get(self.end - 1);

        self.end -= 1;
        Some(cell)
    }
}

#[derive(Debug, Default)]
pub struct Tiles([Pieces; SIZE]);

impl Index<usize> for Tiles {
    type Output = Pieces;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Tiles {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Tiles {
    /// Returns the positions of adjacent pieces
    pub fn adjacent_pieces(&self, (row, col): Position) -> Vec<Position> {
        let row = row as isize;
        let col = col as isize;
        let test: [(isize, isize); 8] = [
            (row-1, col-1),
            (row-1, col),
            (row-1, col+1),
            (row, col-1),
            (row, col+1),
            (row+1, col-1),
            (row+1, col),
            (row+1, col+1),
        ];

        test.into_iter().filter_map(|&pos| {
            self.get(pos).map(|_| (pos.0 as usize, pos.1 as usize))
        }).collect()
    }

    /// Gets the piece at the given position, if there is any
    pub fn get(&self, (row, col): (isize, isize)) -> Option<Piece> {
        if row > 0 && col > 0 {
            *self.0.get(row as usize)
                .map(|r| r.get(col as usize).unwrap_or(&None))
                .unwrap_or(&None)
        }
        else {
            None
        }
    }

    /// Returns the size of each row
    pub fn row_size(&self) -> usize {
        self.0[0].len()
    }

    /// Returns the size of each column
    pub fn col_size(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over each row
    pub fn rows(&self) -> TilesIter {
        TilesIter::new(&self.0, TilesIterTarget::Rows)
    }

    /// Returns an iterator over each row
    pub fn cols(&self) -> TilesIter {
        TilesIter::new(&self.0, TilesIterTarget::Columns)
    }

    /// Returns an iterator over each top-left to bottom-right diagonal
    pub fn diagonals_tlbr(&self) -> TilesIter {
        TilesIter::new(&self.0, TilesIterTarget::DiagonalsTLBR)
    }

    /// Returns an iterator over each top-right to bottom-left diagonal
    pub fn diagonals_trbl(&self) -> TilesIter {
        TilesIter::new(&self.0, TilesIterTarget::DiagonalsTRBL)
    }
}
