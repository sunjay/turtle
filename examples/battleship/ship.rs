use serde::{Deserialize, Serialize};

// This implementation is based on 1990 Milton Bradley version of Battleship
// https://en.wikipedia.org/wiki/Battleship_(game)#Description
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum ShipKind {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl ShipKind {
    // returns the length of the ship
    pub fn size(&self) -> u8 {
        match self {
            Self::Carrier => 5,
            Self::Battleship => 4,
            Self::Cruiser => 3,
            Self::Submarine => 3,
            Self::Destroyer => 2,
        }
    }
}

// Specifies the alignment of a ship in the Grid
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Orientation {
    Horizontal,
    Veritcal,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    pub kind: ShipKind,
    top_left: (u8, u8),
    orientation: Orientation,
}

impl Ship {
    pub fn new(kind: ShipKind, top_left: (u8, u8), orientation: Orientation) -> Self {
        Self {
            kind,
            top_left,
            orientation,
        }
    }
    pub fn coordinates(&self) -> Vec<(u8, u8)> {
        let (x, y) = self.top_left;

        (0..self.kind.size())
            .map(|i| match self.orientation {
                Orientation::Horizontal => (x + i, y),
                Orientation::Veritcal => (x, y + i),
            })
            .collect()
    }
    pub fn is_located_over(&self, pos: &(u8, u8)) -> bool {
        self.coordinates().contains(pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ship_intersection() {
        let carrier = Ship::new(ShipKind::Carrier, (1, 2), Orientation::Veritcal);
        let cspan: Vec<_> = (2..=6).map(|y| (1, y)).collect();

        let battleship = Ship::new(ShipKind::Battleship, (3, 2), Orientation::Horizontal);
        let bspan: Vec<_> = (3..=6).map(|x| (x, 2)).collect();

        for x in 0..10 {
            for y in 0..10 {
                let pos = (x as u8, y as u8);
                if cspan.contains(&pos) {
                    assert!(carrier.is_located_over(&pos));
                } else {
                    assert!(!carrier.is_located_over(&pos));
                }
                if bspan.contains(&pos) {
                    assert!(battleship.is_located_over(&pos));
                } else {
                    assert!(!battleship.is_located_over(&pos));
                }
            }
        }
    }
}
