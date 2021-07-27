use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use turtle::rand::random_range;

use crate::{
    battlestate::{AttackOutcome, BattleState},
    channel::{Channel, Message},
    game::Turn,
    grid::Cell,
};

pub struct Bot {
    channel: Channel,
    state: BattleState,
    turn: Turn,
}

impl Bot {
    pub fn new(port: u16) -> Self {
        Self {
            channel: Channel::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)),
            state: BattleState::new(),
            turn: Turn::Opponent,
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

    fn get_attack_location(&self) -> (u8, u8) {
        // Iterator on positions of all the bombed (Hit, not Destroyed) locations in AttackGrid
        let bombed_locations = self
            .state
            .attack_grid()
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, &cell)| cell == Cell::Bombed)
            .map(|(loc, _)| ((loc as f32 / 10.0).floor() as i32, loc as i32 % 10));

        // Iterate over each bombed location until an attackable position
        // is found in the neighbourhood of the bombed location and return it
        for loc in bombed_locations {
            let attackable = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|n| (n.0 + loc.0, n.1 + loc.1))
                .filter(|pos| matches!(pos.0, 0..=9) && matches!(pos.1, 0..=9))
                .map(|pos| (pos.0 as u8, pos.1 as u8))
                .find(|pos| self.state.can_bomb(&pos));

            if let Some(pos) = attackable {
                return pos;
            }
        }
        // Otherwise return a random attack location if no bombed locations are present
        self.random_attack_location()
    }

    /// Similar to Game::run but without graphics
    pub fn play(&mut self) {
        loop {
            match self.turn {
                Turn::Me => {
                    let attack_location = self.get_attack_location();
                    self.channel.send_message(&Message::AttackCoordinates(attack_location));
                    match self.channel.receive_message() {
                        Message::AttackResult(outcome) => {
                            self.state.set_attack_outcome(&attack_location, outcome);
                            match outcome {
                                AttackOutcome::Miss | AttackOutcome::Destroyed(_) => {
                                    self.turn.flip();
                                }
                                _ => (),
                            }
                        }
                        _ => panic!("Expected Message of AttackResult from Opponent."),
                    }
                }
                Turn::Opponent => match self.channel.receive_message() {
                    Message::AttackCoordinates(p) => {
                        let outcome = self.state.incoming_attack(&p);
                        self.channel.send_message(&Message::AttackResult(outcome));
                        match outcome {
                            AttackOutcome::Miss | AttackOutcome::Destroyed(_) => {
                                self.turn.flip();
                            }
                            AttackOutcome::Hit => (),
                        }
                    }
                    _ => panic!("Expected Message of AttackCoordinates from Opponent"),
                },
            }

            match (self.state.ships_lost, self.state.destroyed_rival_ships) {
                (5, _) => {
                    break;
                }
                (_, 5) => {
                    break;
                }
                (_, _) => continue,
            }
        }
    }
}
