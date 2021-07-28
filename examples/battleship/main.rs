//! Battleship is a two-player strategic guessing game.
//! There are two grids - let's call them - ShipGrid (left) and AttackGrid (right).
//! The ShipGrid, is where the player's fleet of ships is situated and marked.
//! The AttackGrid, is where the opponent's fleet is situated but concealed.
//! The goal is to destroy the opponents fleet by shooting(guessing all tiles) containing a ship of his fleet.
//! Whenever a shot was fired the player gets a feedback: either miss (•) or hit (a red ⚫).
//! That feedback enables some optimization strategies - to guess strategically optimal.
//!
//! You can use the arrow keys (←, ↑, ↓, →) to move the cross-hair in AttackGrid and press the `Enter ⏎` key to attack.
//!
//! To play in single player mode, you can pass `bot` as an argument to the program
//! 
//! $> ./battleship bot
//! $> cargo run --features unstable --example battleship bot # From within the turtle source-code.
//!
//! To play in multiplayer mode, one player needs to act as the server and the other as client.
//! To act as a server, run the program without any arguments.
//! 
//! $> ./battleship # No arguments
//! $> cargo run --features unstable --example battleship # From within the turtle source-code.
//! 
//! This will output something like "Listening on port: <PORT>, Waiting for connection..".
//! As soon as an opponent connects the game starts.
//!
//! If the other player is also within the same LAN, you can share your private IP address and <PORT>
//! which they can use to connect to your game by running the program with these arguments:
//!
//! $> ./battleship <IP:PORT> #  eg: ./battleship 192.168.0.120:35765
//! $> cargo run --features unstable --example battleship
//!
//! If not in same LAN, you can try DynamicDNS or a publicly routable IP address
//! but there are many possible problems that could arise.

// To run, use the command: cargo run --features unstable --example battleship bot
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
