use turtle::rand::Random;

use crate::wall::Wall;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub north: Wall,
    pub east: Wall,
    pub south: Wall,
    pub west: Wall,
}

impl Cell {
    pub fn is_all_closed(&self) -> bool {
        self.north.is_closed()
            && self.east.is_closed()
            && self.south.is_closed()
            && self.west.is_closed()
    }
}

impl Default for Cell {
    fn default() -> Self {
        use Wall::*;

        Self {
            north: Closed,
            east: Closed,
            south: Closed,
            west: Closed,
        }
    }
}

impl Random for Cell {
    fn random() -> Self {
        Self {
            north: Random::random(),
            east: Random::random(),
            south: Random::random(),
            west: Random::random(),
        }
    }
}
