use crate::ship::ShipKind;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Carrier = 0,
    Battleship = 1,
    Cruiser = 2,
    Submarine = 3,
    Destroyer = 4,
    Empty,
    Unattacked,
    Missed,
    Bombed,
    Destroyed,
}

impl ShipKind {
    pub fn to_cell(self) -> Cell {
        match self {
            ShipKind::Carrier => Cell::Carrier,
            ShipKind::Battleship => Cell::Battleship,
            ShipKind::Cruiser => Cell::Cruiser,
            ShipKind::Submarine => Cell::Submarine,
            ShipKind::Destroyer => Cell::Destroyer,
        }
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
