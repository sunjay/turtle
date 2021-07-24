use turtle::{
    event::{Key, PressedState},
    Drawing, Event, Turtle,
};

use crate::{
    battlestate::{AttackOutcome, BattleState, Position},
    channel::{Channel, ChannelType, Message},
    config::Config,
    crosshair::Crosshair,
    grid::Cell,
};

use std::{f64::consts::PI, thread, time::Duration};

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

impl Game {
    pub fn init(channel_type: ChannelType) -> Self {
        let state = BattleState::new();
        let channel = match channel_type {
            ChannelType::Server => Channel::server(),
            ChannelType::Client(addr) => Channel::client(addr),
            ChannelType::ServeOnPort(port) => Channel::serve_on_port(port),
        };
        let turn = match channel_type {
            ChannelType::Client(_) => Turn::Opponent,
            _ => Turn::Me,
        };

        Self { state, channel, turn }
    }

    pub fn draw_cell(&self, cell: Cell, loc: Position, turtle: &mut Turtle) {
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
                self.draw_cell(Cell::Empty, loc, turtle);
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
                self.draw_cell(ship_grid.get(&(x, y)), Position::ShipGrid((x, y)), turtle);
                self.draw_cell(attack_grid.get(&(x, y)), Position::AttackGrid((x, y)), turtle);
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
                        LeftArrow => crosshair.move_left(self),
                        RightArrow => crosshair.move_right(self),
                        UpArrow => crosshair.move_up(self),
                        DownArrow => crosshair.move_down(self),
                        Return => {
                            if let Some(pos) = crosshair.try_bomb() {
                                return pos;
                            }
                        }
                        _ => (),
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
                                self.draw_cell(Cell::Missed, Position::AttackGrid(attack_location), &mut turtle);
                                self.turn.flip();
                            }
                            AttackOutcome::Hit => {
                                self.state.set_attack_outcome(&attack_location, Cell::Bombed);
                                self.draw_cell(Cell::Bombed, Position::AttackGrid(attack_location), &mut turtle);
                            }
                            AttackOutcome::Destroyed(ship) => {
                                self.state.set_destroyed_ship(&ship);
                                ship.coordinates()
                                    .into_iter()
                                    .for_each(|pos| self.draw_cell(Cell::Destroyed, Position::AttackGrid(pos), &mut turtle));
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
                                self.draw_cell(Cell::Missed, Position::ShipGrid(p), &mut turtle);
                                self.turn.flip();
                            }
                            AttackOutcome::Hit => {
                                self.draw_cell(Cell::Bombed, Position::ShipGrid(p), &mut turtle);
                            }
                            AttackOutcome::Destroyed(ship) => {
                                ship.coordinates()
                                    .into_iter()
                                    .for_each(|pos| self.draw_cell(Cell::Destroyed, Position::ShipGrid(pos), &mut turtle));
                                self.turn.flip();
                            }
                        }
                    }
                    _ => panic!("Expected Message of AttackCoordinates from Opponent"),
                },
            }

            match (self.state.ships_lost, self.state.destroyed_rival_ships) {
                (5, _) => {
                    println!("NT");
                    break;
                }
                (_, 5) => {
                    println!("GG");
                    break;
                }
                (_, _) => continue,
            }
        }

        drawing.destroy();
    }
}
