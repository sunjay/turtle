//! A hangman game to guess the hidden word.
// To run this example, use the command: cargo run --features unstable --example hangman
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

use std::io::{Error, Write};

use turtle::{rand, Drawing, Point, Speed, Turtle};

fn main() {
    let mut drawing = Drawing::new();

    // the turtle that is responsible for drawing the hangman design
    let mut hangman = drawing.add_turtle();

    // the turtle that is responsible for drawing the lines on which the letters appear.
    let mut lines = drawing.add_turtle();

    // the turtle that is responsible for drawing the Won or lost smiley.
    let mut wl = drawing.add_turtle();
    wl.hide();

    // the secret word that is to be guessed (the choose function method is part of the [`turtle::rand::RandomSlice`] Trait)
    let secret = rand::choose(WORDS).expect("Failed to choose a random word.");

    // this list stores all the steps of the hangman so that they can be drawn from a for loop or an index.
    let hangman_design = HangmanDesign::default();
    setup(&mut hangman);

    let mut all_guesses = String::new();

    // Loop over each step basically there are two ways to do this either one loops over the steps or one loops over guesses. Both need two nested loops.
    // HangmanDesign has the [`Iterator`] trait implemented which is why we can loop it directly.
    // The `'won:` syntax is to be able to conviniently `break` from both nested loops.
    'won: for step in hangman_design {
        // for each step we only draw it after a wron guess. So the following loops guesses and only `break`s to the for loop on a wrong guess. If all characters have been guessed correctly then it `break`s both loops and continues with a won or lost smiley.
        loop {
            draw_lines(&mut lines, secret, &all_guesses);
            let mut new_guess = String::new();
            ask_for_letter(&mut new_guess).expect("Failed to read letter");
            if secret.contains(&new_guess) {
                println!("This was correct!");
                all_guesses += &new_guess;
                if secret.chars().all(|c| all_guesses.contains(c)) {
                    break 'won;
                }
            } else {
                println!("No that letter is not in the word");
                break;
            }
        }
        step(&mut hangman);
    }
    // Redraw the lines else they could be outdated.
    draw_lines(&mut lines, secret, &all_guesses);
    wl.set_speed(Speed::instant());
    // Draw a circle
    wl.arc_right(100.0, 360.0);
    // Draw the left eye
    teleport_relative(&mut wl, Point { x: 55.0, y: 40.0 });
    wl.arc_right(5.0, 360.0);
    // Draw the right eye
    teleport_relative(&mut wl, Point { x: 85.0, y: 0.0 });
    wl.arc_right(5.0, 360.0);
    // Go to the start of the mouth
    teleport_relative(&mut wl, Point { x: -85.0, y: -80.0 });
    wl.right(90.0);
    // Either draw it smiling or sad
    if secret.chars().all(|c| all_guesses.contains(c)) {
        println!("You Won!");
        wl.right(45.0);
        wl.arc_left(65.0, 90.0);
    } else {
        println!("You Lost!");
        println!("The hidden word was: {}", secret);
        wl.left(45.0);
        wl.arc_right(65.0, 90.0);
    }
}

/// A function to ask the user for a letter
fn ask_for_letter(guess: &mut String) -> Result<(), Error> {
    print!("Please guess a letter: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(guess)?;

    // remove trailing newline characters
    let to_trim = guess.trim_end_matches(&['\r', '\n'][..]).len();
    guess.truncate(to_trim);

    println!("You guessed: {}", &guess);
    Ok(())
}

/// A function to conveniently teleport the turtle relative to its current position.
fn teleport_relative(turtle: &mut Turtle, direction: Point) {
    let cur = turtle.position();
    let new: Point = cur + direction;
    turtle.pen_up();
    turtle.go_to(new);
    turtle.pen_down();
}

/// Draw the lines one for each character. If the character has been guessed already correctly draw a green circle above the line.
fn draw_lines(lines: &mut Turtle, secret: &str, all_guesses: &str) {
    // hide the lines-turtle and go th the startigpoint of the lines.
    lines.reset();
    lines.hide();
    lines.set_speed(Speed::instant());
    lines.right(90.0);
    teleport_relative(lines, Point { x: -300.0, y: 200.0 });

    // draw a line for every character in the secret.
    for letter in secret.chars() {
        if all_guesses.contains(letter) {
            teleport_relative(lines, Point { x: 2.5, y: 10.0 });

            // as turtle is not yet able to draw text draw a green replacement circle.
            lines.left(90.0);
            lines.set_fill_color("green");
            lines.set_pen_color("green");
            lines.begin_fill();
            lines.arc_right(7.5, 360.0);
            lines.end_fill();
            lines.set_pen_color("black");
            lines.right(90.0);

            teleport_relative(lines, Point { x: -2.5, y: -10.0 });

            // print to the terminal
            print!(" {} ", letter);
        } else {
            print!(" _ ");
        }
        lines.forward(20.0);
        teleport_relative(lines, Point { x: 20.0, y: 0.0 });
    }
    println!(); // to switch to the next line and flush the output.
}

// A small word list to choose from
static WORDS: &[&str; 28] = &[
    "poetry",
    "formal",
    "type",
    "expresses",
    "personal",
    "emotions",
    "feelings",
    "typically",
    "spoken",
    "equivalent",
    "lyrics",
    "term",
    "derives",
    "literature",
    "defined",
    "musical",
    "accompaniment",
    "usually",
    "stringed",
    "instrument",
    "term",
    "importance",
    "literary",
    "theory",
    "division",
    "developed",
    "between",
    "categories",
];

/// The struct contains the drawing logic for the hangman graphic
///
/// It is used by creating an instance of a [`HangmanDesign`] struct using [`HangmanDesign::default()`].
/// Since [`HangmanDesign`] implements [`Iterator`] you can directly iterate over the instance using a for loop.
pub struct HangmanDesign {
    state: usize,
    steps: &'static [fn(&mut turtle::Turtle); 9],
}

impl Default for HangmanDesign {
    fn default() -> Self {
        Self {
            state: 0,
            steps: &[hill, mast, bar, support, rope, head, arms, body, legs],
        }
    }
}

impl Iterator for HangmanDesign {
    type Item = &'static fn(&mut turtle::Turtle);

    fn next(&mut self) -> Option<Self::Item> {
        self.state += 1;
        self.steps.get(self.state - 1)
    }
}

pub fn setup(hangman: &mut Turtle) {
    // hide the hangman-turtle and go th the startigpoint of the graphic.
    hangman.hide();
    hangman.set_speed(Speed::instant());
    hangman.set_pen_size(10.0);
    teleport_relative(hangman, Point { x: -300.0, y: -300.0 });
}

fn hill(hangman: &mut Turtle) {
    // start drawing
    hangman.arc_right(100.0, 180.0);
    hangman.left(180.0);
    hangman.arc_left(100.0, 90.0);
    hangman.right(90.0);
}

fn mast(hangman: &mut Turtle) {
    hangman.forward(300.0);
}

fn bar(hangman: &mut Turtle) {
    hangman.right(90.0);
    hangman.forward(150.0);
}

fn support(hangman: &mut Turtle) {
    hangman.backward(100.0);
    hangman.right(135.0);
    hangman.forward(70.710678119);
    hangman.backward(70.710678119);
    hangman.left(135.0);
    hangman.forward(100.0);
}

fn rope(hangman: &mut Turtle) {
    hangman.set_pen_size(3.0);
    hangman.right(90.0);
    hangman.forward(70.0);
}

fn head(hangman: &mut Turtle) {
    hangman.left(90.0);
    hangman.arc_right(30.0, 540.0);
}

fn arms(hangman: &mut Turtle) {
    hangman.left(60.0);
    hangman.forward(100.0);
    hangman.backward(100.0);
    hangman.left(60.0);
    hangman.forward(100.0);
    hangman.backward(100.0);
    hangman.right(30.0);
}

fn body(hangman: &mut Turtle) {
    hangman.forward(100.0);
}

fn legs(hangman: &mut Turtle) {
    hangman.right(20.0);
    hangman.forward(120.0);
    hangman.backward(120.0);
    hangman.left(40.0);
    hangman.forward(120.0);
    hangman.backward(120.0);
    hangman.right(20.0);
}
