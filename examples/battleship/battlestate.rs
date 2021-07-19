use std::{convert::TryInto, fmt::Display, ops::Deref};

use turtle::rand::{choose, random_range};

use super::ship::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    // ship grid
    Carrier = 0,
    Battleship = 1,
    Cruiser = 2,
    Submarine = 3,
    Destroyer = 4,
    Empty,

    // attack grid
    Unattacked,
    Missed,

    // common
    Bombed,
    Destroyed,
}

impl ShipKind {
    fn to_cell(&self) -> Cell {
        match self {
            ShipKind::Carrier => Cell::Carrier,
            ShipKind::Battleship => Cell::Battleship,
            ShipKind::Cruiser => Cell::Cruiser,
            ShipKind::Submarine => Cell::Submarine,
            ShipKind::Destroyer => Cell::Destroyer,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AttackOutcome {
    Miss,
    Hit,
    Destroyed(Ship),
}

struct Grid([[Cell; 10]; 10]);

impl Deref for Grid {
    type Target = [[Cell; 10]; 10];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Grid {
    fn get(&self, pos: &(u8, u8)) -> Cell {
        self.0[pos.0 as usize][pos.1 as usize]
    }
    fn get_mut(&mut self, pos: &(u8, u8)) -> &mut Cell {
        &mut self.0[pos.0 as usize][pos.1 as usize]
    }
    fn count(&mut self, cell: &Cell) -> usize {
        self.iter().flatten().filter(|&c| c == cell).count()
    }
}

pub struct BattleState {
    ship_grid: Grid,
    attack_grid: Grid,
    ships: [Ship; 5],
    pub destroyed_rival_ships: u8,
    pub ships_lost: u8,
}

impl Display for BattleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .ship_grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Cell::Carrier => 'C',
                        Cell::Battleship => 'B',
                        Cell::Cruiser => 'R',
                        Cell::Submarine => 'S',
                        Cell::Destroyer => 'D',
                        _ => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        write!(f, "{}", output.join("\n"))
    }
}

impl BattleState {
    pub fn custom(ships: [Ship; 5]) -> Self {
        let mut ship_grid = Grid {
            0: [[Cell::Empty; 10]; 10],
        };
        ships.iter().for_each(|ship| {
            ship.coordinates().iter().for_each(|pos| {
                *ship_grid.get_mut(pos) = ship.kind.to_cell();
            })
        });
        Self {
            ships,
            ship_grid,
            attack_grid: Grid {
                0: [[Cell::Unattacked; 10]; 10],
            },
            destroyed_rival_ships: 0,
            ships_lost: 0,
        }
    }
    pub fn new() -> Self {
        let (ships, ship_grid) = Self::random_ship_grid();
        Self {
            ships,
            ship_grid,
            attack_grid: Grid {
                0: [[Cell::Unattacked; 10]; 10],
            },
            destroyed_rival_ships: 0,
            ships_lost: 0,
        }
    }
    pub fn incoming_attack(&mut self, pos: &(u8, u8)) -> AttackOutcome {
        let attacked_cell = self.ship_grid.get(pos);
        match attacked_cell {
            Cell::Empty => AttackOutcome::Miss,
            Cell::Carrier | Cell::Battleship | Cell::Cruiser | Cell::Submarine | Cell::Destroyer => {
                let count = self.ship_grid.count(&attacked_cell);
                match count {
                    1 => {
                        let lost_ship = self.ships[attacked_cell as usize];
                        lost_ship
                            .coordinates()
                            .into_iter()
                            .for_each(|loc| *self.ship_grid.get_mut(&loc) = Cell::Destroyed);
                        self.ships_lost += 1;
                        AttackOutcome::Destroyed(lost_ship)
                    }
                    _ => {
                        *self.ship_grid.get_mut(pos) = Cell::Bombed;
                        AttackOutcome::Hit
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn can_bomb(&self, pos: &(u8, u8)) -> bool {
        match self.attack_grid.get(pos) {
            Cell::Bombed | Cell::Destroyed | Cell::Missed => false,
            Cell::Unattacked => true,
            _ => unreachable!(),
        }
    }
    pub fn set_attack_outcome(&mut self, pos: &(u8, u8), cell: Cell) {
        *self.attack_grid.get_mut(pos) = cell;
    }
    pub fn set_destroyed_ship(&mut self, ship: &Ship) {
        ship.coordinates()
            .into_iter()
            .for_each(|pos| *self.attack_grid.get_mut(&pos) = Cell::Destroyed);

        self.destroyed_rival_ships += 1;
    }
    fn random_ship_grid() -> ([Ship; 5], Grid) {
        let ship_types = [
            ShipKind::Carrier,
            ShipKind::Battleship,
            ShipKind::Cruiser,
            ShipKind::Submarine,
            ShipKind::Destroyer,
        ];
        let mut grid = Grid {
            0: [[Cell::Empty; 10]; 10],
        };
        let mut ships = Vec::new();

        for kind in ship_types {
            loop {
                let x: u8 = random_range(0, 9);
                let y: u8 = random_range(0, 9);
                let orient: Orientation = choose(&[Orientation::Horizontal, Orientation::Veritcal]).copied().unwrap();

                let ship_coords = (0..kind.size())
                    .map(|i| match orient {
                        Orientation::Horizontal => (x + i, y),
                        Orientation::Veritcal => (x, y + i),
                    })
                    .collect::<Vec<_>>();

                let no_overlap = ships
                    .iter()
                    .all(|ship: &Ship| ship_coords.iter().all(|pos| !ship.is_located_over(pos)));
                let within_board = ship_coords.iter().all(|pos| matches!(pos.0, 0..=9) && matches!(pos.1, 0..=9));

                if no_overlap && within_board {
                    let ship = Ship::new(
                        kind,
                        ShipPosition::new(ship_coords.first().copied().unwrap(), ship_coords.last().copied().unwrap()),
                    );
                    ships.push(ship);
                    ship_coords.iter().for_each(|pos| {
                        *grid.get_mut(pos) = kind.to_cell();
                    });
                    break;
                }
            }
        }

        (ships.try_into().unwrap(), grid)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn battle_actions() {
        let ships = [
            Ship {
                kind: ShipKind::Carrier,
                position: ShipPosition {
                    top_left: (2, 4),
                    bottom_right: (2, 8),
                },
            },
            Ship {
                kind: ShipKind::Battleship,
                position: ShipPosition {
                    top_left: (1, 0),
                    bottom_right: (4, 0),
                },
            },
            Ship {
                kind: ShipKind::Cruiser,
                position: ShipPosition {
                    top_left: (5, 2),
                    bottom_right: (7, 2),
                },
            },
            Ship {
                kind: ShipKind::Submarine,
                position: ShipPosition {
                    top_left: (8, 4),
                    bottom_right: (8, 6),
                },
            },
            Ship {
                kind: ShipKind::Destroyer,
                position: ShipPosition {
                    top_left: (6, 7),
                    bottom_right: (9, 7),
                },
            },
        ];
        // Player's ship grid      Opponent's ship grid
        //   0 1 2 3 4 5 6 7 8 9     0 1 2 3 4 5 6 7 8 9
        // 0 . B B B B . . . . .   0 . . . . . . . . . .
        // 1 . . . . . . . . . .   1 . . . . . S S S . .
        // 2 . . . . . R R R . .   2 . . . . . . D D . .
        // 3 . . . . . . . . . .   3 . . . . . . . . . .
        // 4 . . C . . . . . S .   4 . . . . B B B B . .
        // 5 . . C . . . . . S .   5 . . . C C C C C . .
        // 6 . . C . . . . . S .   6 . . . . . . . . . .
        // 7 . . C . . . D D . .   7 . . . R R R . . . .
        // 8 . . C . . . . . . .   8 . . . . . . . . . .
        // 9 . . . . . . . . . .   9 . . . . . . . . . .
        let mut state = BattleState::custom(ships);
        // turn 1: player attacks (2, 2) - misses
        state.set_attack_outcome(&(2, 2), Cell::Missed);
        assert_eq!(state.attack_grid.get(&(2, 2)), Cell::Missed);
        // turn 2: opponent attacks (6, 7) - hits
        let outcome = state.incoming_attack(&(6, 7));
        assert_eq!(outcome, AttackOutcome::Hit);
        assert_eq!(state.ship_grid.get(&(6, 7)), Cell::Bombed);
        // turn 3: opponent attacks (again) (7, 7) - destroys D
        let outcome = state.incoming_attack(&(7, 7));
        assert_eq!(outcome, AttackOutcome::Destroyed(ships[4]));
        assert_eq!(state.ship_grid.get(&(7, 7)), Cell::Destroyed);
        assert_eq!(state.ship_grid.get(&(6, 7)), Cell::Destroyed);
        assert_eq!(state.ships_lost, 1);
        // turn 4: player attacks (7, 2) - hits
        state.set_attack_outcome(&(7, 2), Cell::Bombed);
        assert_eq!(state.attack_grid.get(&(7, 2)), Cell::Bombed);
        // turn 5: player attacks (6, 2) - destroys D
        state.set_destroyed_ship(&Ship {
            kind: ShipKind::Destroyer,
            position: ShipPosition {
                top_left: (6, 2),
                bottom_right: (7, 2),
            },
        });
        assert_eq!(state.attack_grid.get(&(6, 2)), Cell::Destroyed);
        assert_eq!(state.attack_grid.get(&(7, 2)), Cell::Destroyed);
        assert_eq!(state.destroyed_rival_ships, 1);
    }
}
