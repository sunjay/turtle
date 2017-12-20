//! To run a LOGO program, run:
//!     cargo run --example logo_interpreter -- my_logo_program.txt

extern crate turtle;

use turtle::Turtle;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut turtle = Turtle::new();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("No file provided");
        std::process::exit(1);
    }

    let path = &args[1];
    let file = File::open(path).expect("Could not open provided program");
    let mut reader = BufReader::new(file);
    loop {
        let mut buffer = String::new();
        let read_bytes = reader
            .read_line(&mut buffer)
            .expect("Unable read input file");
        if read_bytes == 0 {
            // Reached EOF, stop interpreter
            break;
        }

        let cmd_args = buffer.split_whitespace();
        handle_command(&mut turtle, cmd_args);
    }
}

fn handle_command<'a, A: Iterator<Item = &'a str>>(turtle: &mut Turtle, mut args: A) {
    while let Some(command) = args.next() {
        match command {
            "fd" | "forward" => {
                let distance = parse_distance(
                    args.next()
                        .expect("Expected a distance value after fd/forward command"),
                );
                turtle.forward(distance);
            },
            "bk" | "back" => {
                let distance = parse_distance(
                    args.next()
                        .expect("Expect a distance value after bk/back command"),
                );
                turtle.backward(distance);
            },
            "lt" | "left" => {
                let distance = parse_distance(
                    args.next()
                        .expect("Expect a distance value after lt/left command"),
                );
                turtle.left(distance);
            },
            "rt" | "right" => {
                let distance = parse_distance(
                    args.next()
                        .expect("Expect a distance value after rt/right command"),
                );
                turtle.right(distance);
            },
            _ => unimplemented!("Use of invalid or unsupported LOGO command"),
        }
    }
}

fn parse_distance(s: &str) -> f64 {
    let dist: f64 = s.parse().expect("Not a valide number");
    dist
}
