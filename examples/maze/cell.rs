use turtle::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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
        self.north.is_closed() && self.east.is_closed() && self.south.is_closed() && self.west.is_closed()
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

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        let mut cell = Cell::default();
        cell.north = rng.gen();
        cell.east = rng.gen();
        cell.south = rng.gen();
        cell.west = rng.gen();
        cell
    }
}
