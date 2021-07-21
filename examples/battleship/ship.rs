use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ShipPosition {
    pub top_left: (u8, u8),
    pub bottom_right: (u8, u8),
}

impl ShipPosition {
    pub fn new(top_left: (u8, u8), bottom_right: (u8, u8)) -> Self {
        Self { top_left, bottom_right }
    }
}

// Based on https://en.wikipedia.org/wiki/Battleship_(game)#Description
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum ShipKind {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl ShipKind {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Veritcal,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    pub kind: ShipKind,
    pub position: ShipPosition,
}

impl Ship {
    pub fn new(kind: ShipKind, position: ShipPosition) -> Self {
        Self { kind, position }
    }
    pub fn orientation(&self) -> Orientation {
        let diff_x = self.position.top_left.0 as i32 - self.position.bottom_right.0 as i32;
        let diff_y = self.position.top_left.1 as i32 - self.position.bottom_right.1 as i32;
        match (diff_x, diff_y) {
            (0, _) => Orientation::Veritcal,
            (_, 0) => Orientation::Horizontal,
            (_, _) => unreachable!(),
        }
    }
    pub fn is_located_over(&self, pos: &(u8, u8)) -> bool {
        let collinear = {
            (pos.0 as i32 - self.position.top_left.0 as i32) * (self.position.top_left.1 as i32 - self.position.bottom_right.1 as i32)
                - (self.position.top_left.0 as i32 - self.position.bottom_right.0 as i32) * (pos.1 as i32 - self.position.top_left.1 as i32)
                == 0
        };
        let x_within_bounds = (self.position.top_left.0..=self.position.bottom_right.0).contains(&pos.0);
        let y_within_bounds = (self.position.top_left.1..=self.position.bottom_right.1).contains(&pos.1);
        collinear && x_within_bounds && y_within_bounds
    }
    pub fn coordinates(&self) -> Vec<(u8, u8)> {
        let orientation = self.orientation();
        let x = self.position.top_left.0;
        let y = self.position.top_left.1;

        (0..self.kind.size())
            .map(|i| match orientation {
                Orientation::Horizontal => (x + i, y),
                Orientation::Veritcal => (x, y + i),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ship_orientation() {
        let carrier = Ship {
            kind: ShipKind::Carrier,
            position: ShipPosition {
                top_left: (1, 2),
                bottom_right: (1, 6),
            },
        };

        let battleship = Ship {
            kind: ShipKind::Battleship,
            position: ShipPosition {
                top_left: (3, 2),
                bottom_right: (6, 2),
            },
        };

        assert_eq!(carrier.orientation(), Orientation::Veritcal);
        assert_eq!(battleship.orientation(), Orientation::Horizontal);
    }

    #[test]
    fn ship_intersection() {
        let carrier = Ship {
            kind: ShipKind::Carrier,
            position: ShipPosition {
                top_left: (1, 2),
                bottom_right: (1, 6),
            },
        };
        let cspan: Vec<_> = (2..=6).map(|y| (1, y)).collect();

        let battleship = Ship {
            kind: ShipKind::Battleship,
            position: ShipPosition {
                top_left: (3, 2),
                bottom_right: (6, 2),
            },
        };
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
