// To run, use the command: cargo run --features unstable --example battleship
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

mod battlestate;
mod bot;
mod channel;
mod config;
mod game;
mod ship;

use std::{net::TcpListener, thread, time::Duration};

use game::{Game, Player};

use crate::bot::Bot;

fn get_available_tcp_port() -> u16 {
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
            let mut game = Game::init(Player::Server);
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
            let mut game = Game::init(Player::ServeOnPort(port));
            game.run();
            handle.join().unwrap();
        }
        addr => {
            let mut game = Game::init(Player::Client(addr));
            game.run();
        }
    }
}
