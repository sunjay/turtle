use turtle::rand::{Rand, Rng};

use wall::Wall;

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
        use self::Wall::*;

        Self {
            north: Closed,
            east: Closed,
            south: Closed,
            west: Closed,
        }
    }
}

impl Rand for Cell {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut cell = Cell::default();
        cell.north = Wall::rand(rng);
        cell.east = Wall::rand(rng);
        cell.south = Wall::rand(rng);
        cell.west = Wall::rand(rng);
        cell
    }
}
