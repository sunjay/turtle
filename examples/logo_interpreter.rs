//! To run a LOGO program, run:
//!     cargo run --example logo_interpreter -- .\examples\my_logo_program.txt

extern crate turtle;

use turtle::Turtle;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed(4);

    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {
        let source= parse_source(&parse_file(&args[1]));
        parse_string(&source, &mut turtle);
    }
    else {
        println!("Pass in a LOGO command txt file.");
    }
}

fn parse_string(source: &str, turtle: &mut Turtle) {
    let ops: Vec<&str> = source.split(' ').collect();

    let mut ins_pointer = 0;

    while let Some(op) = ops.get(ins_pointer) {
        match *op {
            "fd" | "forward" => if let Some(dist) = ops.get(ins_pointer + 1) {
                let dist: f64 = dist.parse().unwrap();
                turtle.forward(dist as f64);
                ins_pointer += 1;
            },
            "bk" | "back" => if let Some(dist) = ops.get(ins_pointer + 1) {
                let dist: f64 = dist.parse().unwrap();
                turtle.backward(dist as f64);
                ins_pointer += 1;
            },
            "lt" | "left" => if let Some(dist) = ops.get(ins_pointer + 1) {
                let dist: f64 = dist.parse().unwrap();
                turtle.left(dist as f64);
                ins_pointer += 1;
            },
            "rt" | "right" => if let Some(dist) = ops.get(ins_pointer + 1) {
                let dist: f64 = dist.parse().unwrap();
                turtle.right(dist as f64);
                ins_pointer += 1;
            },
            _ => println!("command: {}", op),
        }
        ins_pointer += 1;
    }
}

fn parse_file(file_name: &str) -> String {
    let path = Path::new(file_name);
    let mut file = match File::open(&path) {
        Err(why) => {
            println!("FILE ERROR");
            panic!("Error: {}", why);
        },
        Ok(file) => file,
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => {},
        Err(why) => {
            println!("READ ERROR");
            panic!("Error: {}", why);
        },
    };
    text
}

fn parse_source(source: &str) -> String {
    // Transform the single string to a vector of strings by splitting at whitespaces.
    let src: Vec<&str> = source.split_whitespace().collect();
    // Join the Vector to form a single String with even spaces.
    let src = src.join(" ").to_string();
    src
}