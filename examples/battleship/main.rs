// To run, use the command: cargo run --features unstable --example battleship
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

mod battlestate;
mod channel;
mod config;
mod game;
mod ship;

use game::{Game, Player};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let player = match args.len() {
        0 => unreachable!(),
        1 => Player::Server,
        _ => Player::Client(&args[1]),
    };
    let mut game = Game::init(player);
    game.run();
}
