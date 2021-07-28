use crate::{grid::Cell, ship::ShipKind};
use turtle::Color;

pub struct Config {}

impl Config {
    pub const EMPTY_COLOR: &'static str = "#55dde0";
    pub const UNATTACKED_COLOR: &'static str = "#55dde0";
    pub const CARRIER_COLOR: &'static str = "#fde74c";
    pub const BATTLESHIP_COLOR: &'static str = "#f4d58d";
    pub const CRUISER_COLOR: &'static str = "#947757";
    pub const SUBMARINE_COLOR: &'static str = "#9bc53d";
    pub const DESTROYER_COLOR: &'static str = "#238cf4";
    pub const MISSED_COLOR: &'static str = "#33658a";
    pub const BOMBED_COLOR: &'static str = "#f26419";
    pub const DESTROYED_COLOR: &'static str = "#349c9e";

    pub const CELL_SIZE: f64 = 40.0;
    pub const SPACE_BETWEEN_GRIDS: f64 = 50.0;

    pub const SHIP_GRID_TOP_LEFT: (f64, f64) = (-Self::SPACE_BETWEEN_GRIDS / 2.0 - 10.0 * Self::CELL_SIZE, 5.0 * Self::CELL_SIZE);
    pub const ATTACK_GRID_TOP_LEFT: (f64, f64) = (Self::SPACE_BETWEEN_GRIDS / 2.0, 5.0 * Self::CELL_SIZE);

    pub const MISSED_CIRCLE_RADIUS: f64 = 0.25 * Self::CELL_SIZE * 0.5;
    pub const BOMBED_CIRCLE_RADIUS: f64 = 0.75 * Self::CELL_SIZE * 0.5;

    pub const CROSSHAIR_SIZE: f64 = 0.2 * Self::CELL_SIZE;
    pub const CROSSHAIR_PEN_SIZE: f64 = 4.0;
    pub const CROSSHAIR_COLOR: &'static str = "#f26419";
    pub const DISABLED_CROSSHAIR_COLOR: &'static str = "#000000";

    pub fn cell_color(cell: &Cell) -> Color {
        match cell {
            Cell::Ship(ShipKind::Carrier) => Self::CARRIER_COLOR.into(),
            Cell::Ship(ShipKind::Battleship) => Self::BATTLESHIP_COLOR.into(),
            Cell::Ship(ShipKind::Cruiser) => Self::CRUISER_COLOR.into(),
            Cell::Ship(ShipKind::Submarine) => Self::SUBMARINE_COLOR.into(),
            Cell::Ship(ShipKind::Destroyer) => Self::DESTROYER_COLOR.into(),
            Cell::Empty => Self::EMPTY_COLOR.into(),
            Cell::Unattacked => Self::UNATTACKED_COLOR.into(),
            Cell::Missed => Self::MISSED_COLOR.into(),
            Cell::Bombed => Self::BOMBED_COLOR.into(),
            Cell::Destroyed => Self::DESTROYED_COLOR.into(),
        }
    }
}
