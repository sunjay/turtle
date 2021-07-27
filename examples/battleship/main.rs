//! Battleship is a two-player strategic guessing game.
//! There are two grids - let's call them - ShipGrid and AttackGrid.
//! ShipGrid, located on the left hand side, is where the player's fleet of ships is situated and marked.
//! AttackGrid, located on the right hand side, is where the opponent's fleet is situated but concealed.
//! Players alternate turns calling "shots" at the other player's ships.
//! You can use arrow keys (←, ↑, ↓, →) to move crosshair in AttackGrid and press `Enter ⏎` key to attack.
//! The objective of the game is to destroy the opposing player's fleet.
//!
//! This game can be played in single player mode as well as multiplayer.
//! To play in single player mode, you can pass `bot` as an argument to the program
//! $> ./battleship bot
//!
//! To play in multiplayer mode, one player needs to acts as a server and the other as client.
//! To act as a server, run the program without any arguments.
//! $> ./battleship # No arguments
//! This will output something like "Listening on port: <PORT>, Waiting for connection..".
//!
//! If the other player is also within the same LAN, you can share your private IP address and <PORT>
//! which they can use to connect with you by running the program with these arguments:
//! $> ./battleship <IP:PORT> #  eg: ./battleship 192.168.0.120:35765
//!
//! If not in same LAN, you can try DynamicDNS or a publicly routable IP address.

// To run, use the command: cargo run --features unstable --example battleship
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

mod battlestate;
mod bot;
mod channel;
mod config;
mod crosshair;
mod game;
mod grid;
mod ship;
mod utils;

use bot::Bot;
use channel::ChannelType;
use game::Game;
use std::{thread, time::Duration};
use utils::*;

fn main() {
    let opt = parse_args();
    match opt {
        Opt::Server => {
            let mut game = Game::init(ChannelType::Server);
            game.run();
        }
        Opt::Client(addr) => {
            let mut game = Game::init(ChannelType::Client(addr));
            game.run();
        }
        Opt::PlayWithBot => {
            // Create a TCP listener on a free port and use it to make a game server.
            // The game server will start listening on that port while we spawn
            // a bot instance in a separate thread which would later connnect to the server.
            let listener = get_tcp_listener();
            let port = listener.local_addr().unwrap().port();
            let handle = thread::spawn(move || {
                // delay to let the game server start listening on port
                thread::sleep(Duration::from_millis(10));
                let mut bot = Bot::new(port);
                bot.play();
            });
            let mut game = Game::init(ChannelType::UseListener(listener));
            game.run();
            handle.join().unwrap();
        }
    }
}
