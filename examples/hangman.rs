//! A hangman game to guess the hidden word.

use turtle::{Drawing, Point, Speed, Turtle};

// To run this example, use the command: cargo run --features unstable --example flower
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

fn main() {
    let mut drawing = Drawing::new();
    let mut hangman = drawing.add_turtle();
    let mut lines = drawing.add_turtle();

    let secret = "trainstation";

    hangman.hide();
    hangman.set_speed(Speed::instant());
    hangman.set_pen_size(10.0);
    hangman.pen_up();
    hangman.go_to(Point { x: -300.0, y: -300.0 });
    hangman.pen_down();

    let list = &[hill, mast, bar, support, rope, head, arms, body, legs];

    for step in list.iter() {
        step(&mut hangman);
    }

    lines.hide();
    lines.set_speed(Speed::instant());
    lines.right(90.0);
    lines.pen_up();
    lines.go_to(Point{x:-300.0, y:200.0});
    lines.pen_down();

    for _letter in secret.chars(){
        lines.forward(20.0);
        lines.pen_up();
        lines.forward(20.0);
        lines.pen_down();
    }
}

fn hill(hangman: &mut Turtle) {
    hangman.arc_right(100.0, 180.0);
    hangman.left(180.0);
    hangman.arc_left(100.0, 90.0);
    hangman.right(90.0);
}

fn mast(hangman: &mut Turtle) {
    hangman.forward(300.0);
    //turtle.
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

fn arms(hangman: &mut Turtle){
    hangman.left(60.0);
    hangman.forward(100.0);
    hangman.backward(100.0);
    hangman.left(60.0);
    hangman.forward(100.0);
    hangman.backward(100.0);
    hangman.right(30.0);
}

fn body(hangman: &mut Turtle){
    hangman.forward(100.0);
}

fn legs(hangman: &mut Turtle){
    hangman.right(20.0);
    hangman.forward(120.0);
    hangman.backward(120.0);
    hangman.left(40.0);
    hangman.forward(120.0);
    hangman.backward(120.0);
    hangman.right(20.0);
}
