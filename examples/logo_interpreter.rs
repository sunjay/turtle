//! To run a LOGO program, run:
//!     cargo run --example logo_interpreter -- my_logo_program.txt

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Interactive LOGO Mode.\nType the commands in stdin.\nPress '^C' (cmd/ctrl+C) to exit");
        let stdin = io::stdin();
        interpret(&mut turtle, stdin)
    } else {
        let path = &args[1];
        let file = File::open(path).expect("Could not open provided program");
        interpret(&mut turtle, file)
    }
}

fn interpret<R: io::Read>(turtle: &mut Turtle, input: R) {
    let mut reader = BufReader::new(input);
    loop {
        let mut buffer = String::new();
        let read_bytes = reader.read_line(&mut buffer).expect("Unable read input");
        if read_bytes == 0 {
            // Reached EOF, break loop
            break;
        }
        let cmd_args = buffer.split_whitespace();
        handle_command(turtle, cmd_args);
    }
}

fn handle_command<'a, A: Iterator<Item = &'a str>>(turtle: &mut Turtle, mut args: A) {
    while let Some(command) = args.next() {
        match command {
            "fd" | "forward" => {
                let distance = parse_distance(args.next().expect("Expected a distance value after fd/forward command"));
                turtle.forward(distance);
            }
            "bk" | "back" => {
                let distance = parse_distance(args.next().expect("Expect a distance value after bk/back command"));
                turtle.backward(distance);
            }
            "lt" | "left" => {
                let distance = parse_distance(args.next().expect("Expect a distance value after lt/left command"));
                turtle.left(distance);
            }
            "rt" | "right" => {
                let distance = parse_distance(args.next().expect("Expect a distance value after rt/right command"));
                turtle.right(distance);
            }
            "home" => {
                turtle.home();
            }
            "setx" => {
                let x_pos = parse_distance(args.next().expect("No expression found"));
                turtle.set_x(x_pos);
            }
            "sety" => {
                let y_pos = parse_distance(args.next().expect("No expression found"));
                turtle.set_y(y_pos);
            }
            "setheading" | "seth" => {
                let expr = parse_distance(args.next().expect("No expression found"));
                turtle.set_heading(expr);
            }
            "showturtle" | "st" => {
                turtle.show();
            }
            "hideturtle" | "ht" => {
                turtle.hide();
            }
            "clean" => {
                turtle.clear();
            }
            "clearscreen" | "cs" => {
                turtle.clear();
                turtle.home();
            }
            "pendown" | "pd" => {
                turtle.pen_down();
            }
            "penup" | "pu" => {
                turtle.pen_up();
            }
            "setpensize" | "setwidth" | "setpw" => {
                let value = parse_distance(args.next().expect("No value found"));
                turtle.set_pen_size(value);
            }
            _ => unimplemented!("Use of invalid or unsupported LOGO command"),
        }
    }
}

fn parse_distance(s: &str) -> f64 {
    let dist: f64 = s.parse().expect("Not a valid number");
    dist
}
