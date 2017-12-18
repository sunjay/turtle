use std::ops::Deref;
use std::collections::HashSet;

use turtle::Color;

/// (Row, Column)
pub type Position = (usize, usize);

type Tiles = [[Option<Piece>; 8]; 8];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    A,
    B,
}

impl Piece {
    pub fn name(self) -> &'static str {
        match self {
            Piece::A => "red",
            Piece::B => "blue",
        }
    }

    pub fn other(self) -> Self {
        match self {
            Piece::A => Piece::B,
            Piece::B => Piece::A,
        }
    }

    pub fn color(self) -> Color {
        match self {
            Piece::A => "#f44336".into(),
            Piece::B => "#2196F3".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    current: Piece,
    /// None - empty tile
    /// Some(Piece::A) - occupied by piece A
    /// Some(Piece::B) - occupied by piece B
    ///
    /// Each array in Board is a row of the board
    tiles: Tiles,
    valid_moves: HashSet<Position>
}

impl Deref for Board {
    type Target = Tiles;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl Board {
    pub fn new() -> Self {
        let mut tiles: Tiles = Default::default();
        tiles[3][3] = Some(Piece::A);
        tiles[3][4] = Some(Piece::B);
        tiles[4][3] = Some(Piece::B);
        tiles[4][4] = Some(Piece::A);
        let current = Piece::A;

        let mut board = Self {
            current,
            tiles,
            valid_moves: HashSet::new(),
        };
        board.update_valid_moves(current);
        board
    }

    pub fn current(&self) -> Piece {
        self.current
    }

    pub fn valid_moves(&self) -> &HashSet<Position> {
        &self.valid_moves
    }

    pub fn is_valid_move(&self, position: &Position) -> bool {
        self.valid_moves.contains(position)
    }

    /// Returns the tiles that were flipped
    pub fn play_piece(&mut self, pos: Position) -> Vec<Position> {
        if self.is_valid_move(&pos) {
            assert!(self[pos.0][pos.1].is_none(), "Valid move was not an empty tile!");
            self.tiles[pos.0][pos.1] = Some(self.current);
            let flipped = self.flip_tiles(pos);

            self.current = self.current.other();

            //TODO: When nested method calls are enabled, this can be done in one line
            // Link: https://github.com/rust-lang/rust/issues/44100
            let current = self.current;
            self.update_valid_moves(current);
            flipped
        }
        else {
            unreachable!("Game should check for whether a valid move was used before playing it");
        }
    }

    fn flip_tiles(&mut self, (row, col): Position) -> Vec<Position> {
        let piece = self.current;
        assert_eq!(self.tiles[row][col], Some(piece));
        let other = piece.other();
        let rows = self.tiles.len() as isize;
        let cols = self.tiles[0].len() as isize;

        let mut flipped = Vec::new();
        for (adj_row, adj_col) in self.adjacent_positions((row, col)) {
            if self.tiles[adj_row][adj_col] == Some(other)
                && self.find_piece((row, col), (adj_row, adj_col), piece) {
                // Perform flips
                let delta_row = adj_row as isize - row as isize;
                let delta_col = adj_col as isize - col as isize;
                let mut curr_row = adj_row as isize;
                let mut curr_col = adj_col as isize;
                while curr_row >= 0 && curr_row < rows && curr_col >= 0 && curr_col < cols {
                    let current = &mut self.tiles[curr_row as usize][curr_col as usize];
                    if *current == Some(other) {
                        *current = Some(piece);
                        flipped.push((curr_row as usize, curr_col as usize));
                    }
                    curr_row += delta_row;
                    curr_col += delta_col;
                }
            }
        }
        flipped
    }

    fn update_valid_moves(&mut self, piece: Piece) {
        self.valid_moves.clear();

        // Explanation: A valid move is an empty tile which has `piece` in a vertical, horizontal,
        // or diagonal line from it with only `piece.other()` between the empty tile and piece.
        // Example: E = empty, p = piece, o = other piece
        //      A B C D E F G H I J K
        //      E E o o o p o p p E o
        //  Tile A is *not* a valid move. Tile B is a valid move for p. None of the other tiles are
        //  valid moves for p.
        // Algorithm: For each empty tile, look for at least one adjacent `other` piece. If one is
        // found, look for another `piece` in that direction that isn't preceeded by an empty tile.

        let other = piece.other();
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                // Only empty tiles can be valid moves
                if tile.is_some() {
                    continue;
                }

                for (row, col) in self.adjacent_positions((i, j)) {
                    // Look for at least one `other` tile before finding `piece`
                    if self.tiles[row][col] == Some(other)
                        && self.find_piece((i, j), (row, col), piece) {
                        self.valid_moves.insert((i, j));
                        // Don't want to keep searching this tile now that we've added it
                        break;
                    }
                }
            }
        }

        // We need to shrink to fit because clear does not reduce the capacity and we do not want
        // to leak memory by allowing the valid_moves Vec to grow uncontrollably
        self.valid_moves.shrink_to_fit();
    }

    /// Searches in the direction of the given target starting from the target. Returns true if it
    /// finds piece AND only encounters piece.other() along the way.
    fn find_piece(&self, pos: Position, (target_row, target_col): Position, piece: Piece) -> bool {
        let other = piece.other();
        let rows = self.tiles.len() as isize;
        let cols = self.tiles[0].len() as isize;

        let delta_row = target_row as isize - pos.0 as isize;
        let delta_col = target_col as isize - pos.1 as isize;

        let mut curr_row = target_row as isize + delta_row;
        let mut curr_col = target_col as isize + delta_col;
        while curr_row >= 0 && curr_row < rows && curr_col >= 0 && curr_col < cols {
            let current = self.tiles[curr_row as usize][curr_col as usize];
            curr_row += delta_row;
            curr_col += delta_col;
            if current == Some(other) {
                continue;
            }
            else if current == Some(piece) {
                return true;
            }
            else {
                return false;
            }
        }
        return false;
    }

    //TODO: Replace return type with `impl Iterator<Item=Position>` when the "impl Trait"
    // feature is stable.
    fn adjacent_positions(&self, (row, col): Position) -> Vec<Position> {
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();
        [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)].into_iter()
            .map(|&(r, c)| (row as isize + r, col as isize + c))
            .filter(|&(r, c)| r >= 0 && c >= 0 && r < rows as isize && c < cols as isize)
            .map(|(r, c)| (r as usize, c as usize))
            .collect()
    }
}
