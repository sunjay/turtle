use battlestate::*;
use game::{Game, Player};
use turtle::{rand::random_range, Drawing};

mod battlestate;
mod channel;
mod config;
mod game;
mod ship;

fn main() {
    let mut drawing = Drawing::new();
    let args: Vec<String> = std::env::args().collect();
    let player = match args.len() {
        0 => Player::Server,
        _ => Player::Client(&args[0]),
    };
    let mut game = Game::new(player, drawing.add_turtle());
    game.draw_board();
}
//#[derive(Debug, Copy, Clone)]
//enum Turn {
//    Player,
//    Opponent,
//}
//
//impl Turn {
//    fn flip(&mut self) {
//        match self {
//            Turn::Player => *self = Turn::Opponent,
//            Turn::Opponent => *self = Turn::Player,
//        }
//    }
//    fn next(&mut self, attacker: &mut BattleState, defender: &mut BattleState) {
//        let attack_pos = random_attack_target(&attacker);
//        println!("{:?} attacks {:?}", self, attack_pos);
//        let outcome = defender.incoming_attack(&attack_pos);
//        println!("Outcome: {:#?}", outcome);
//        match outcome {
//            AttackOutcome::Hit => attacker.set_attack_outcome(&attack_pos, Cell::Bombed),
//            AttackOutcome::Miss => {
//                attacker.set_attack_outcome(&attack_pos, Cell::Missed);
//                self.flip();
//            }
//            AttackOutcome::Destroyed(ship) => {
//                attacker.set_destroyed_ship(&ship);
//                self.flip();
//            }
//        }
//    }
//}
//
//fn random_attack_target(state: &BattleState) -> (u8, u8) {
//    loop {
//        let x = random_range(0, 9);
//        let y = random_range(0, 9);
//        if state.can_bomb(&(x, y)) {
//            return (x, y);
//        }
//    }
//}
//
//fn john() {
//    println!("Welcome to Battleship!");
//    let mut player_state = battlestate::BattleState::new();
//    println!("PLAYER: \n{}", player_state);
//    let mut opponent_state = battlestate::BattleState::new();
//    println!("OPPONENT: \n{}", opponent_state);
//
//    let mut turn = Turn::Player;
//
//    loop {
//        match turn {
//            Turn::Player => {
//                turn.next(&mut player_state, &mut opponent_state);
//            }
//            Turn::Opponent => {
//                turn.next(&mut opponent_state, &mut player_state);
//            }
//        }
//
//        match (player_state.ships_lost, player_state.destroyed_rival_ships) {
//            (5, _) => {
//                println!("Opponent Won!");
//                break;
//            }
//            (_, 5) => {
//                println!("Player Won!");
//                break;
//            }
//            (_, _) => continue,
//        }
//    }
//}
//
