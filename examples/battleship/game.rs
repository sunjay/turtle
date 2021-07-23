use turtle::{
    event::{Key, PressedState},
    rand::random_range,
    Drawing, Event, Turtle,
};

use crate::{
    battlestate::{AttackOutcome, BattleState, Cell},
    channel::{Channel, Message},
    config::Config,
};

use std::f64::consts::PI;

enum Turn {
    Me,
    Opponent,
}

impl Turn {
    fn flip(&mut self) {
        match self {
            Turn::Me => *self = Turn::Opponent,
            Turn::Opponent => *self = Turn::Me,
        }
    }
}

pub struct Game {
    state: BattleState,
    channel: Channel,
    turn: Turn,
}

pub enum Player<'a> {
    Server,
    Client(&'a str),
}

struct Crosshair<'a> {
    pos: (u8, u8),
    state: &'a BattleState,
    turtle: &'a mut Turtle,
}

impl<'a> Crosshair<'a> {
    fn random_attackable_location(state: &BattleState) -> (u8, u8) {
        loop {
            let x = random_range(0, 9);
            let y = random_range(0, 9);
            if state.can_bomb(&(x, y)) {
                return (x, y);
            }
        }
    }

    fn draw_crosshair(pos: (u8, u8), turtle: &mut Turtle) {
        let (x, y) = Config::ATTACK_GRID_TOP_LEFT;
        turtle.set_pen_color(Config::TARGET_COLOR);
        turtle.set_pen_size(Config::CROSSHAIR_PEN_SIZE);
        let start = (
            x + Config::CELL_SIZE * (0.5 + pos.1 as f64),
            y - Config::CELL_SIZE * (0.5 + pos.0 as f64),
        );
        turtle.pen_up();
        turtle.go_to(start);
        turtle.pen_down();
        turtle.set_heading(0.0);
        for _ in 0..4 {
            turtle.forward(Config::CROSSHAIR_SIZE);
            turtle.pen_up();
            turtle.backward(Config::CROSSHAIR_SIZE);
            turtle.pen_down();
            turtle.right(90.0);
        }
        turtle.set_pen_color("black");
        turtle.set_pen_size(1.0);
    }

    fn new(state: &'a BattleState, turtle: &'a mut Turtle) -> Self {
        let pos = Self::random_attackable_location(state);
        Self::draw_crosshair(pos, turtle);
        Self { pos, state, turtle }
    }

    fn move_left(&mut self) {
        // TODO: use inclusive range
        let new_y = (0..self.pos.1).rev().find(|&y| self.state.can_bomb(&(self.pos.0, y)));
        if let Some(y) = new_y {
            let cell = self.state.attack_grid().get(&self.pos);
            Game::draw_cell(cell, Position::AttackGrid(self.pos), &mut self.turtle);

            let new_pos = (self.pos.0, y);
            Self::draw_crosshair(new_pos, self.turtle);
            self.pos = new_pos;
        }
    }
    fn move_right(&mut self) {
        let new_y = (self.pos.1 + 1..10).find(|&y| self.state.can_bomb(&(self.pos.0, y)));
        if let Some(y) = new_y {
            let cell = self.state.attack_grid().get(&self.pos);
            Game::draw_cell(cell, Position::AttackGrid(self.pos), &mut self.turtle);

            let new_pos = (self.pos.0, y);
            Self::draw_crosshair(new_pos, self.turtle);
            self.pos = new_pos;
        }
    }
    fn move_up(&mut self) {
        let new_x = (0..self.pos.0).rev().find(|&x| self.state.can_bomb(&(x, self.pos.1)));
        if let Some(x) = new_x {
            let cell = self.state.attack_grid().get(&self.pos);
            Game::draw_cell(cell, Position::AttackGrid(self.pos), &mut self.turtle);

            let new_pos = (x, self.pos.1);
            Self::draw_crosshair(new_pos, self.turtle);
            self.pos = new_pos;
        }
    }
    fn move_down(&mut self) {
        let new_x = (self.pos.0 + 1..10).find(|&x| self.state.can_bomb(&(x, self.pos.1)));
        if let Some(x) = new_x {
            let cell = self.state.attack_grid().get(&self.pos);
            Game::draw_cell(cell, Position::AttackGrid(self.pos), &mut self.turtle);

            let new_pos = (x, self.pos.1);
            Self::draw_crosshair(new_pos, self.turtle);
            self.pos = new_pos;
        }
    }
    fn lock_target(&mut self) -> (u8, u8) {
        let cell = self.state.attack_grid().get(&self.pos);
        Game::draw_cell(cell, Position::AttackGrid(self.pos), &mut self.turtle);
        return self.pos;
    }
}

enum Position {
    ShipGrid((u8, u8)),
    AttackGrid((u8, u8)),
}

impl Position {
    fn get(self) -> (u8, u8) {
        match self {
            Self::ShipGrid(p) => p,
            Self::AttackGrid(p) => p,
        }
    }
}

impl Game {
    pub fn init(player: Player) -> Self {
        let state = BattleState::new();
        let channel = match player {
            Player::Server => Channel::server(),
            Player::Client(addr) => Channel::client(addr),
        };
        let turn = match player {
            Player::Client(_) => Turn::Opponent,
            Player::Server => Turn::Me,
        };

        Self { state, channel, turn }
    }

    fn draw_cell(cell: Cell, pos: Position, turtle: &mut Turtle) {
        fn draw_circle(turtle: &mut Turtle, diameter: f64) {
            turtle.set_heading(0.0);
            turtle.begin_fill();
            for _ in 0..360 {
                turtle.forward(PI * diameter / 360.0);
                turtle.right(1.0);
            }
            turtle.end_fill();
        }
        fn draw_square(turtle: &mut Turtle, size: f64) {
            turtle.set_heading(0.0);
            turtle.begin_fill();
            for _ in 0..4 {
                turtle.forward(size);
                turtle.right(90.0);
            }
            turtle.end_fill();
        }
        let (x, y) = match pos {
            Position::ShipGrid(_) => Config::SHIP_GRID_TOP_LEFT,
            Position::AttackGrid(_) => Config::ATTACK_GRID_TOP_LEFT,
        };

        let pos = pos.get();

        match cell {
            Cell::Missed | Cell::Bombed => {
                let diameter = if cell == Cell::Missed {
                    Config::MISSED_CIRCLE_DIAMETER
                } else {
                    Config::BOMBED_CIRCLE_DIAMETER
                };
                let start = (
                    x + Config::CELL_SIZE * (pos.1 as f64 + 0.5),
                    y - Config::CELL_SIZE * pos.0 as f64 - (Config::CELL_SIZE / 2.0 - diameter / 2.0),
                );
                turtle.pen_up();
                turtle.go_to(start);
                turtle.pen_down();
                turtle.set_fill_color(Config::cell_color(&cell));
                draw_circle(turtle, diameter);
            }
            _ => {
                let start = (x + Config::CELL_SIZE * pos.1 as f64, y - Config::CELL_SIZE * pos.0 as f64);
                turtle.pen_up();
                turtle.go_to(start);
                turtle.pen_down();
                turtle.set_fill_color(Config::cell_color(&cell));
                draw_square(turtle, Config::CELL_SIZE);
            }
        }
    }

    fn draw_board(&self, turtle: &mut Turtle) {
        let ship_grid = self.state.ship_grid();
        let attack_grid = self.state.attack_grid();

        for x in 0..10 {
            for y in 0..10 {
                Self::draw_cell(ship_grid.get(&(x, y)), Position::ShipGrid((x, y)), turtle);
                Self::draw_cell(attack_grid.get(&(x, y)), Position::AttackGrid((x, y)), turtle);
            }
        }
    }
    fn random_attack_location(&self) -> (u8, u8) {
        loop {
            let x = random_range(0, 9);
            let y = random_range(0, 9);
            if self.state.can_bomb(&(x, y)) {
                return (x, y);
            }
        }
    }

    fn get_attack_location(&self, drawing: &mut Drawing, turtle: &mut Turtle) -> (u8, u8) {
        let mut crosshair = Crosshair::new(&self.state, turtle);
        loop {
            while let Some(event) = drawing.poll_event() {
                use Key::{DownArrow, LeftArrow, Return, RightArrow, UpArrow};
                match event {
                    Event::Key(key, PressedState::Pressed) => match key {
                        LeftArrow => crosshair.move_left(),
                        RightArrow => crosshair.move_right(),
                        UpArrow => crosshair.move_up(),
                        DownArrow => crosshair.move_down(),
                        Return => {
                            return crosshair.lock_target();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut drawing = Drawing::new();
        let mut turtle = drawing.add_turtle();

        turtle.hide();
        turtle.set_speed("instant");
        self.draw_board(&mut turtle);

        loop {
            match self.turn {
                Turn::Me => {
                    let attack_location = self.get_attack_location(&mut drawing, &mut turtle);
                    self.channel.send_message(&Message::AttackCoordinates(attack_location));
                    match self.channel.receive_message() {
                        Message::AttackResult(outcome) => match outcome {
                            AttackOutcome::Miss => {
                                self.state.set_attack_outcome(&attack_location, Cell::Missed);
                                Self::draw_cell(Cell::Missed, Position::AttackGrid(attack_location), &mut turtle);
                                self.turn.flip();
                            }
                            AttackOutcome::Hit => {
                                self.state.set_attack_outcome(&attack_location, Cell::Bombed);
                                Self::draw_cell(Cell::Bombed, Position::AttackGrid(attack_location), &mut turtle);
                            }
                            AttackOutcome::Destroyed(ship) => {
                                self.state.set_destroyed_ship(&ship);
                                ship.coordinates()
                                    .into_iter()
                                    .for_each(|pos| Self::draw_cell(Cell::Destroyed, Position::AttackGrid(pos), &mut turtle));
                                self.turn.flip();
                            }
                        },
                        _ => panic!("Expected Message of AttackResult from Opponent."),
                    }
                }
                Turn::Opponent => match self.channel.receive_message() {
                    Message::AttackCoordinates(p) => {
                        let outcome = self.state.incoming_attack(&p);
                        self.channel.send_message(&Message::AttackResult(outcome));
                        match outcome {
                            AttackOutcome::Miss => {
                                Self::draw_cell(Cell::Missed, Position::ShipGrid(p), &mut turtle);
                                self.turn.flip();
                            }
                            AttackOutcome::Hit => {
                                Self::draw_cell(Cell::Bombed, Position::ShipGrid(p), &mut turtle);
                            }
                            AttackOutcome::Destroyed(ship) => {
                                ship.coordinates()
                                    .into_iter()
                                    .for_each(|pos| Self::draw_cell(Cell::Destroyed, Position::ShipGrid(pos), &mut turtle));
                                self.turn.flip();
                            }
                        }
                    }
                    _ => panic!("Expected Message of AttackCoordinates from Opponent"),
                },
            }

            match (self.state.ships_lost, self.state.destroyed_rival_ships) {
                (5, _) => {
                    println!("Nice try.");
                    break;
                }
                (_, 5) => {
                    println!("GG!");
                    break;
                }
                (_, _) => continue,
            }
        }
    }
}
