use turtle::{Rand, Rng};

use wall::Wall;

#[derive(Debug, Clone, Copy)]
pub enum CellMarker {
    Start,
    Finish,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub north: Wall,
    pub east: Wall,
    pub south: Wall,
    pub west: Wall,
    pub marker: Option<CellMarker>,
}

impl Default for Cell {
    fn default() -> Self {
        use self::Wall::*;

        Self {
            north: Closed,
            east: Closed,
            south: Closed,
            west: Closed,
            marker: None,
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
