use turtle::{Rand, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wall {
    Open,
    Closed,
}

impl Rand for Wall {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        // Taken from Rand impl for bool in rand crate
        if rng.gen::<u8>() & 1 == 1 {
            Wall::Open
        }
        else {
            Wall::Closed
        }
    }
}

impl Wall {
    pub fn is_open(&self) -> bool {
        match *self {
            Wall::Open => true,
            Wall::Closed => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        match *self {
            Wall::Open => false,
            Wall::Closed => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub north: Wall,
    pub east: Wall,
    pub south: Wall,
    pub west: Wall
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
