use turtle::rand::{distributions::{Distribution, Standard}, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wall {
    Open,
    Closed,
}

impl Distribution<Wall> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Wall {
        // Taken from impl for bool in rand crate
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
