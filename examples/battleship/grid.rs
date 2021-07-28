use crate::ship::ShipKind;
use std::{fmt::Display, ops::Deref};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Ship(ShipKind),
    /// clear cell on ShipGrid
    Empty,
    /// clear cell on AttackGrid
    Unattacked,
    Missed,
    Bombed,
    /// Denotes a ship cell of a completely destroyed ship
    Destroyed,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Ship(ShipKind::Carrier) => write!(f, "C"),
            Cell::Ship(ShipKind::Battleship) => write!(f, "B"),
            Cell::Ship(ShipKind::Cruiser) => write!(f, "R"),
            Cell::Ship(ShipKind::Submarine) => write!(f, "S"),
            Cell::Ship(ShipKind::Destroyer) => write!(f, "D"),
            Cell::Empty | Cell::Unattacked => write!(f, "."),
            Cell::Missed => write!(f, ","),
            Cell::Bombed => write!(f, "*"),
            Cell::Destroyed => write!(f, "#"),
        }
    }
}

impl ShipKind {
    pub fn to_cell(self) -> Cell {
        Cell::Ship(self)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Grid([[Cell; 10]; 10]);

impl Deref for Grid {
    type Target = [[Cell; 10]; 10];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..10 {
            for j in 0..10 {
                write!(f, "{}", self.get(&(j, i)))?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(cell: Cell) -> Self {
        Self { 0: [[cell; 10]; 10] }
    }
    pub fn get(&self, pos: &(u8, u8)) -> Cell {
        self.0[pos.0 as usize][pos.1 as usize]
    }
    pub fn get_mut(&mut self, pos: &(u8, u8)) -> &mut Cell {
        &mut self.0[pos.0 as usize][pos.1 as usize]
    }
    pub fn count(&mut self, cell: &Cell) -> usize {
        self.iter().flatten().filter(|&c| c == cell).count()
    }
}
