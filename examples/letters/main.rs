//! Draws text on the screen using the turtle.
//!
//! This example draws the text introduced as a first parameter
//! for example:
//!   cargo run --example letters 'Wow! So cool!'
//! Draw the text 'Wow! So cool!'
//! the optional second parameter is a font size, for example:
//!   cargo run --example letters 'Big text!' 50
//! Draw the text 'Big text!' with font size 50
//! if you don't insert any parameter:
//!   cargo run --example letters
//! this will draw a simple "hello, world!"

use std::env;
use std::process;

mod a_to_d;
mod diacritics;
mod e_to_i;
mod j_to_o;
mod numbers;
mod p_to_t;
mod punctuation;
mod u_to_z;

use a_to_d::*;
use diacritics::*;
use e_to_i::*;
use j_to_o::*;
use numbers::*;
use p_to_t::*;
use punctuation::*;
use u_to_z::*;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed("fast");

    let (text, font_size) = parse_args();

    // Go to the start of the text
    go_to_initial_position(&mut turtle, &text, font_size);

    for character in text.chars() {
        match character {
            '0' => zero(&mut turtle, font_size),
            '1' => one(&mut turtle, font_size),
            '2' => two(&mut turtle, font_size),
            '3' => three(&mut turtle, font_size),
            '4' => four(&mut turtle, font_size),
            '5' => five(&mut turtle, font_size),
            '6' => six(&mut turtle, font_size),
            '7' => seven(&mut turtle, font_size),
            '8' => eight(&mut turtle, font_size),
            '9' => nine(&mut turtle, font_size),
            'a' | 'A' => a(&mut turtle, font_size),
            'b' | 'B' => b(&mut turtle, font_size),
            'c' | 'C' => c(&mut turtle, font_size),
            'd' | 'D' => d(&mut turtle, font_size),
            'e' | 'E' => e(&mut turtle, font_size),
            'f' | 'F' => f(&mut turtle, font_size),
            'g' | 'G' => g(&mut turtle, font_size),
            'h' | 'H' => h(&mut turtle, font_size),
            'i' | 'I' => i(&mut turtle, font_size),
            'j' | 'J' => j(&mut turtle, font_size),
            'k' | 'K' => k(&mut turtle, font_size),
            'l' | 'L' => l(&mut turtle, font_size),
            'm' | 'M' => m(&mut turtle, font_size),
            'n' | 'N' => n(&mut turtle, font_size),
            'ñ' | 'Ñ' => {
                tilde(&mut turtle, font_size);
                with_title(&mut turtle, font_size);
                n(&mut turtle, font_size);
            }
            'o' | 'O' => o(&mut turtle, font_size),
            'p' | 'P' => p(&mut turtle, font_size),
            'q' | 'Q' => q(&mut turtle, font_size),
            'r' | 'R' => r(&mut turtle, font_size),
            's' | 'S' => s(&mut turtle, font_size),
            't' | 'T' => t(&mut turtle, font_size),
            'u' | 'U' => u(&mut turtle, font_size),
            'v' | 'V' => v(&mut turtle, font_size),
            'w' | 'W' => w(&mut turtle, font_size),
            'x' | 'X' => x(&mut turtle, font_size),
            'y' | 'Y' => y(&mut turtle, font_size),
            'z' | 'Z' => z(&mut turtle, font_size),
            'á' | 'Á' | 'é' | 'É' | 'í' | 'Í' | 'ó' | 'Ó' | 'ú' | 'Ú' => {
                acutte(&mut turtle, font_size);
                with_accute(&mut turtle, font_size);
                match character {
                    'á' | 'Á' => a(&mut turtle, font_size),
                    'é' | 'É' => e(&mut turtle, font_size),
                    'í' | 'Í' => i(&mut turtle, font_size),
                    'ó' | 'Ó' => o(&mut turtle, font_size),
                    _ => u(&mut turtle, font_size),
                };
            }
            '´' => acutte(&mut turtle, font_size),
            '\'' => apostrophe(&mut turtle, font_size),
            ':' => colon(&mut turtle, font_size),
            ',' => comma(&mut turtle, font_size),
            '.' => dot(&mut turtle, font_size),
            '!' => exclamation(&mut turtle, font_size),
            '¡' => inverted_exclamation(&mut turtle, font_size),
            '¿' => inverted_question(&mut turtle, font_size),
            '?' => question(&mut turtle, font_size),
            ';' => semicolon(&mut turtle, font_size),
            '~' => tilde(&mut turtle, font_size),
            character if character.is_whitespace() => space(&mut turtle, font_size),
            _ => {
                println!("We still don't have an implementation for the '{}' character!", character);
                println!("but you can add it!:");
                println!("https://github.com/sunjay/turtle#contributing");
                question(&mut turtle, font_size);
            }
        }
        turtle.pen_down();
    }
    turtle.hide();
}

fn go_to_initial_position(turtle: &mut Turtle, text: &str, font_size: f64) {
    turtle.pen_up();
    turtle.left(90.0);
    for _ in text.chars() {
        turtle.forward(font_size / 2.0);
    }
    turtle.right(90.0);
    turtle.pen_down();
}

fn with_title(turtle: &mut Turtle, font_size: f64) {
    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(1.5 * font_size);
    turtle.right(90.0);
    turtle.pen_down();
}

fn with_accute(turtle: &mut Turtle, font_size: f64) {
    turtle.left(90.0);
    turtle.forward(font_size);
    turtle.pen_down();
    turtle.right(90.0);
}

/// Parses the command line arguments or exits with a help message if there was
/// an error
fn parse_args() -> (String, f64) {
    let mut args = env::args();

    // Skip the first argument (the executable name)
    args.next();

    // First argument is the text to draw
    let text = match args.next() {
        Some(text) if text == "--help" => print_help(),

        // This can produce any text, including the empty string
        Some(text) => text,
        // Default to `Hello, World!`
        None => "Hello, World!".to_string(),
    };

    // Second argument is the font size
    let font_size: f64 = match args.next() {
        Some(text) if text == "--help" => print_help(),

        Some(font_size) => match font_size.parse() {
            Ok(font_size) => {
                if font_size >= 1.0 {
                    font_size
                } else {
                    println!("Font size argument must be at least 1.0");
                    println!();
                    print_help();
                }
            }

            Err(err) => {
                println!("Font size argument must be a valid number: {}", err);
                println!();
                print_help();
            }
        },

        // Default to a font size of 20
        None => 20.0,
    };

    // Not expecting any other arguments
    if args.next().is_some() {
        print_help()
    }

    (text, font_size)
}

/// Prints the help message and then exits
///
/// `!` is the "never type" and it means this function never returns
fn print_help() -> ! {
    println!("Draws text on the screen using the turtle.");
    println!();
    println!("EXAMPLES:");
    println!("  cargo run --example letters 'Wow! So cool!'");
    println!("    Draw the text 'Wow! So cool!'");
    println!();
    println!("  cargo run --example letters 'Big text!' 50");
    println!("    Draw the text 'Big text!' with font size 50");
    println!();
    println!("  cargo run --example letters -- --help");
    println!("    Show this help information");
    println!();
    println!("FLAGS:");
    println!("  --help  show this help information");

    process::exit(0)
}
