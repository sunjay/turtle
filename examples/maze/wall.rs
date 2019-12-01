use turtle::rand::Random;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wall {
    Open,
    Closed,
}

impl Random for Wall {
    fn random() -> Self {
        if Random::random() {
            Wall::Open
        } else {
            Wall::Closed
        }
    }
}

impl Wall {
    pub fn is_closed(&self) -> bool {
        match *self {
            Wall::Open => false,
            Wall::Closed => true,
        }
    }
}
