use wall::Wall;

#[derive(Debug, Clone, Copy)]
pub enum CellMarker {
    Start,
    Finish,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub north: Wall,
    pub east: Wall,
    pub south: Wall,
    pub west: Wall,
    pub marker: Option<CellMarker>,
}

impl Default for Cell {
    fn default() -> Self {
        use self::Wall::*;

        Self {
            north: Closed,
            east: Closed,
            south: Closed,
            west: Closed,
            marker: None,
        }
    }
}
