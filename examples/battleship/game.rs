use turtle::Turtle;

use crate::{
    battlestate::{BattleState, Cell},
    channel::Channel,
    config::Config,
};

pub struct Game {
    state: BattleState,
    turtle: Turtle,
    //channel: Channel,
}

pub enum Player<'a> {
    Server,
    Client(&'a str),
}

enum GridType {
    ShipGrid,
    AttackGrid,
}

impl Game {
    pub fn new(player: Player, turtle: Turtle) -> Self {
        let mut turtle = turtle;
        turtle.set_speed("instant");
        turtle.hide();
        let state = BattleState::new();
        //let channel = match player {
        //    Server => Channel::server(),
        //    Client(addr) => Channel::client(addr),
        //};
        Self {
            state,
            turtle, /*channel*/
        }
    }

    pub fn draw_cell(&mut self, cell: Cell, pos: (u8, u8), grid: GridType) {
        let (x, y) = match grid {
            GridType::ShipGrid => Config::SHIP_GRID_TOP_LEFT,
            GridType::AttackGrid => Config::ATTACK_GRID_TOP_LEFT,
        };
        let start = (x + Config::CELL_SIZE * pos.1 as f64, y - Config::CELL_SIZE * pos.0 as f64);

        self.turtle.pen_up();
        self.turtle.go_to(start);
        self.turtle.pen_down();
        self.turtle.set_heading(0.0); // face East
        self.turtle.set_fill_color(Config::cell_color(&cell));
        self.turtle.begin_fill();
        for _ in 0..4 {
            self.turtle.forward(Config::CELL_SIZE);
            self.turtle.right(90.0);
        }
        self.turtle.end_fill();
    }

    pub fn draw_board(&mut self) {
        let ship_grid = self.state.ship_grid();
        println!("{}", self.state);
        let attack_grid = self.state.attack_grid();

        for x in 0..10 {
            for y in 0..10 {
                self.draw_cell(ship_grid.get(&(x, y)), (x, y), GridType::ShipGrid);
                self.draw_cell(attack_grid.get(&(x, y)), (x, y), GridType::AttackGrid);
            }
        }
    }
}
