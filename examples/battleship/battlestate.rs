use serde::{Deserialize, Serialize};
use std::{convert::TryInto, fmt::Display};
use turtle::rand::{choose, random_range};

use crate::{
    grid::{Cell, Grid},
    ship::{Orientation, Ship, ShipKind},
};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AttackOutcome {
    Miss,
    Hit,
    Destroyed(Ship),
}

#[derive(Copy, Clone)]
pub enum Position {
    ShipGrid((u8, u8)),
    AttackGrid((u8, u8)),
}

impl Position {
    pub fn get(self) -> (u8, u8) {
        match self {
            Self::ShipGrid(p) => p,
            Self::AttackGrid(p) => p,
        }
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
    pub fn new() -> Self {
        let (ships, ship_grid) = Self::random_ship_grid();
        Self {
            ships,
            ship_grid,
            attack_grid: Grid::new(Cell::Unattacked),
            destroyed_rival_ships: 0,
            ships_lost: 0,
        }
    }
    pub fn incoming_attack(&mut self, pos: &(u8, u8)) -> AttackOutcome {
        let attacked_cell = self.ship_grid.get(pos);
        match attacked_cell {
            Cell::Empty => AttackOutcome::Miss,
            Cell::Carrier | Cell::Battleship | Cell::Cruiser | Cell::Submarine | Cell::Destroyer => {
                let standing_ship_parts = self.ship_grid.count(&attacked_cell);
                match standing_ship_parts {
                    1 => {
                        // If the attack is on the last standing ship part,
                        // change all the Cells of the Ship to Destroyed
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
    pub fn set_attack_outcome(&mut self, attacked_pos: &(u8, u8), outcome: AttackOutcome) {
        match outcome {
            AttackOutcome::Miss => *self.attack_grid.get_mut(attacked_pos) = Cell::Missed,
            AttackOutcome::Hit => *self.attack_grid.get_mut(attacked_pos) = Cell::Bombed,
            AttackOutcome::Destroyed(ship) => {
                for pos in ship.coordinates() {
                    *self.attack_grid.get_mut(&pos) = Cell::Destroyed;
                }
                self.destroyed_rival_ships += 1;
            }
        }
    }
    fn random_ship_grid() -> ([Ship; 5], Grid) {
        let ship_types = [
            ShipKind::Carrier,
            ShipKind::Battleship,
            ShipKind::Cruiser,
            ShipKind::Submarine,
            ShipKind::Destroyer,
        ];
        let mut grid = Grid::new(Cell::Empty);
        let mut ships = Vec::new();

        // Randomly select a position and orientation for a ship type to create a Ship
        // Check if the ship doesn't overlap with other ships already added to Grid
        // Check if the ship is within the Grid bounds
        // If the above two conditions are met, add the ship to the Grid
        // And proceed with next ship type
        for kind in ship_types {
            loop {
                let x: u8 = random_range(0, 9);
                let y: u8 = random_range(0, 9);
                let orient: Orientation = choose(&[Orientation::Horizontal, Orientation::Veritcal]).copied().unwrap();

                let ship = Ship::new(kind, (x, y), orient);

                let no_overlap = ships
                    .iter()
                    .all(|other: &Ship| other.coordinates().iter().all(|pos| !ship.is_located_over(pos)));

                let within_board = ship
                    .coordinates()
                    .iter()
                    .all(|pos| matches!(pos.0, 0..=9) && matches!(pos.1, 0..=9));

                if no_overlap && within_board {
                    ships.push(ship);
                    ship.coordinates().iter().for_each(|pos| {
                        *grid.get_mut(pos) = kind.to_cell();
                    });
                    break;
                }
            }
        }

        (ships.try_into().unwrap(), grid)
    }
    pub fn ship_grid(&self) -> &'_ Grid {
        &self.ship_grid
    }
    pub fn attack_grid(&self) -> &'_ Grid {
        &self.attack_grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn custom_battlestate(ships: [Ship; 5]) -> BattleState {
        let mut ship_grid = Grid::new(Cell::Empty);
        ships.iter().for_each(|ship| {
            ship.coordinates().iter().for_each(|pos| {
                *ship_grid.get_mut(pos) = ship.kind.to_cell();
            })
        });
        BattleState {
            ships,
            ship_grid,
            attack_grid: Grid::new(Cell::Unattacked),
            destroyed_rival_ships: 0,
            ships_lost: 0,
        }
    }

    #[test]
    fn battle_actions() {
        let ships = [
            Ship::new(ShipKind::Carrier, (2, 4), Orientation::Veritcal),
            Ship::new(ShipKind::Battleship, (1, 0), Orientation::Horizontal),
            Ship::new(ShipKind::Cruiser, (5, 2), Orientation::Horizontal),
            Ship::new(ShipKind::Submarine, (8, 4), Orientation::Veritcal),
            Ship::new(ShipKind::Destroyer, (6, 7), Orientation::Horizontal),
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
        let mut state = custom_battlestate(ships);
        // turn 1: player attacks (2, 2) - misses
        state.set_attack_outcome(&(2, 2), AttackOutcome::Miss);
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
        state.set_attack_outcome(&(7, 2), AttackOutcome::Hit);
        assert_eq!(state.attack_grid.get(&(7, 2)), Cell::Bombed);
        // turn 5: player attacks (6, 2) - destroys D
        state.set_attack_outcome(
            &(6, 2),
            AttackOutcome::Destroyed(Ship::new(ShipKind::Destroyer, (6, 2), Orientation::Horizontal)),
        );
        assert_eq!(state.attack_grid.get(&(6, 2)), Cell::Destroyed);
        assert_eq!(state.attack_grid.get(&(7, 2)), Cell::Destroyed);
        assert_eq!(state.destroyed_rival_ships, 1);
    }
}
