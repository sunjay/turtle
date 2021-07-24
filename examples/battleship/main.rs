//! Battleship board Game
//! Usage:
//! $> ./battleship # No arguments - outputs a TCP port waiting for the opponent to connect.
//! $> ./battleship <ip:port> #  - connects to a server with the specified ip port
//! $> ./battleship bot #  play with computer (single player).

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

use bot::Bot;
use channel::ChannelType;
use game::Game;
use std::{net::TcpListener, thread, time::Duration};

pub fn get_available_tcp_port() -> u16 {
    for port in 49152..=65535 {
        if TcpListener::bind(&format!("127.0.0.1:{}", port)).is_ok() {
            return port;
        }
    }
    panic!("No ports available!");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = match args.len() {
        0 => unreachable!(),
        1 => "",
        _ => &args[1],
    };

    match config {
        "" => {
            let mut game = Game::init(ChannelType::Server);
            game.run();
        }
        "bot" => {
            // May fail due to TOCTOU
            let port = get_available_tcp_port();
            let handle = thread::spawn(move || {
                // delay to let game server bind to port
                thread::sleep(Duration::from_millis(10));
                let mut bot = Bot::new(port);
                bot.play();
            });
            let mut game = Game::init(ChannelType::ServeOnPort(port));
            game.run();
            handle.join().unwrap();
        }
        addr => {
            let mut game = Game::init(ChannelType::Client(addr));
            game.run();
        }
    }
}
