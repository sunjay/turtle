//! To run a LOGO program, run:
//!     cargo run --example logo_interpreter -- my_logo_program.txt

extern crate turtle;

use turtle::Turtle;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

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
        let read_bytes = reader.read_line(&mut buffer)
            .expect("Unable read input file");
        if read_bytes == 0 {
            // Reached EOF, stop interpreter
            break;
        }

        let mut cmd_args = buffer.split_whitespace().collect();
        handle_command(&mut turtle, &mut cmd_args);
    }
}

fn handle_command(turtle: &mut Turtle, args: &mut VecDeque<&str>) {
    if args.is_empty() {
        return;
    }
    // We already checked if args is empty, so we can unwrap here without fear
    match args.pop_front().unwrap() {
        "fd" | "forward" => {
            let distance = parse_distance(args.pop_front()
                .expect("Expected a distance value after fd/forward command"));
            turtle.forward(distance);
        },
        "bk" | "back" => {
            let distance = parse_distance(args.pop_front()
                .expect("Expect a distance value after bk/back command"));
            turtle.backward(distance);
        },
        "lt" | "left" => {
            let distance = parse_distance(args.pop_front()
                .expect("Expect a distance value after lt/left command"));
            turtle.left(distance);
        },
        "rt" | "right" => {
            let distance = parse_distance(args.pop_front()
                .expect("Expect a distance value after rt/right command"));
            turtle.right(distance);
        },
        _ => unimplemented!("Use of invalid or unsupported LOGO command"),
    }
    // Parse any remaining commands on this line
    handle_command(turtle, args);
}

fn parse_distance(s: &str) -> f64 {
    let dist: f64 = s.parse().expect("Not a valide number");
    dist
}
