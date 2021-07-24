use turtle::{
    event::{Key, PressedState},
    Drawing, Event, Turtle,
};

use crate::{
    battlestate::{AttackOutcome, BattleState, Cell},
    channel::{Channel, Message},
    config::Config,
};

use std::f64::consts::PI;

pub enum Turn {
    Me,
    Opponent,
}

impl Turn {
    pub fn flip(&mut self) {
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
    ServeOnPort(u16),
}

enum CrosshairType {
    LockTarget,
    Disabled,
}

struct Crosshair<'a> {
    pos: (u8, u8),
    state: &'a BattleState,
    turtle: &'a mut Turtle,
}

impl<'a> Crosshair<'a> {
    fn new(state: &'a BattleState, turtle: &'a mut Turtle, last_bombed_pos: Option<(u8, u8)>) -> Self {
        let pos;
        if let Some(bombed_pos) = last_bombed_pos {
            pos = bombed_pos;
            Self::draw_crosshair(pos, turtle, CrosshairType::Disabled);
        } else {
            pos = (4, 4);
            Self::draw_crosshair(pos, turtle, CrosshairType::LockTarget);
        }
        Self { pos, state, turtle }
    }

    fn draw_crosshair(pos: (u8, u8), turtle: &mut Turtle, crosshair: CrosshairType) {
        let (x, y) = Config::ATTACK_GRID_TOP_LEFT;
        turtle.set_pen_color(if matches!(crosshair, CrosshairType::Disabled) {
            Config::DISABLED_CROSSHAIR_COLOR
        } else {
            Config::CROSSHAIR_COLOR
        });
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

    fn move_to(&mut self, pos: (u8, u8)) {
        //remove crosshair by redrawing the cell
        let cell = self.state.attack_grid().get(&self.pos);
        Game::draw_cell(cell, Position::AttackGrid(self.pos), self.turtle);

        let crosshair = match self.state.can_bomb(&pos) {
            true => CrosshairType::LockTarget,
            false => CrosshairType::Disabled,
        };
        Self::draw_crosshair(pos, self.turtle, crosshair);
        self.pos = pos;
    }

    fn move_left(&mut self) {
        if self.pos.1 > 0 {
            self.move_to((self.pos.0, self.pos.1 - 1));
        }
    }

    fn move_right(&mut self) {
        if self.pos.1 < 9 {
            self.move_to((self.pos.0, self.pos.1 + 1));
        }
    }

    fn move_up(&mut self) {
        if self.pos.0 > 0 {
            self.move_to((self.pos.0 - 1, self.pos.1));
        }
    }

    fn move_down(&mut self) {
        if self.pos.0 < 9 {
            self.move_to((self.pos.0 + 1, self.pos.1));
        }
    }

    fn try_bomb(&mut self) -> Option<(u8, u8)> {
        if self.state.can_bomb(&self.pos) {
            return Some(self.pos);
        }
        None
    }
}

#[derive(Copy, Clone)]
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
            Player::ServeOnPort(port) => Channel::serve_on_port(port),
        };
        let turn = match player {
            Player::Client(_) => Turn::Opponent,
            _ => Turn::Me,
        };

        Self { state, channel, turn }
    }

    fn draw_cell(cell: Cell, loc: Position, turtle: &mut Turtle) {
        fn draw_circle(turtle: &mut Turtle, diameter: f64) {
            let pen_color = turtle.pen_color();
            turtle.set_pen_color("transparent");
            turtle.set_heading(0.0);
            turtle.begin_fill();
            for _ in 0..360 {
                turtle.forward(PI * diameter / 360.0);
                turtle.right(1.0);
            }
            turtle.end_fill();
            turtle.set_pen_color(pen_color);
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
        let (x, y) = match loc {
            Position::ShipGrid(_) => Config::SHIP_GRID_TOP_LEFT,
            Position::AttackGrid(_) => Config::ATTACK_GRID_TOP_LEFT,
        };

        let pos = loc.get();

        match cell {
            Cell::Missed | Cell::Bombed => {
                Self::draw_cell(Cell::Empty, loc, turtle);
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

    fn get_attack_location(&self, drawing: &mut Drawing, turtle: &mut Turtle, last_bombed_location: Option<(u8, u8)>) -> (u8, u8) {
        let mut crosshair = Crosshair::new(&self.state, turtle, last_bombed_location);
        loop {
            while let Some(event) = drawing.poll_event() {
                use Key::{DownArrow, LeftArrow, Return, RightArrow, UpArrow};
                if let Event::Key(key, PressedState::Pressed) = event {
                    match key {
                        LeftArrow => crosshair.move_left(),
                        RightArrow => crosshair.move_right(),
                        UpArrow => crosshair.move_up(),
                        DownArrow => crosshair.move_down(),
                        Return => {
                            if let Some(pos) = crosshair.try_bomb() {
                                return pos;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut drawing = Drawing::new();
        let mut turtle = drawing.add_turtle();
        let mut last_bombed_location = None;

        turtle.hide();
        turtle.set_speed("instant");
        self.draw_board(&mut turtle);

        loop {
            match self.turn {
                Turn::Me => {
                    let attack_location = self.get_attack_location(&mut drawing, &mut turtle, last_bombed_location);
                    last_bombed_location = Some(attack_location);
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

        drawing.destroy();
    }
}
