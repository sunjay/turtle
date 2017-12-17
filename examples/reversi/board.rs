use turtle::Color;

/// None - empty tile
/// Some(Piece::A) - occupied by piece A
/// Some(Piece::B) - occupied by piece B
///
/// Each array in Board is a row of the board
pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    A,
    B,
}

impl Piece {
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
