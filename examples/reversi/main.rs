//! Reversi
//!
//! https://en.wikipedia.org/wiki/Reversi

extern crate turtle;

mod board;

use std::f64::consts::PI;

use turtle::{Turtle, Event, Color};
use turtle::event::{MouseButton};

use board::{Board, Piece};

#[derive(Debug, Clone)]
struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub rows: usize,
    pub cols: usize,
    pub tile_width: f64,
    pub tile_height: f64,
}

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_background_color("#B3E5FC");
    turtle.set_pen_color("#757575");
    turtle.set_pen_size(2.0);
    turtle.set_speed(8);

    let width = 580.0;
    let height = 580.0;
    let board = Board::new();
    let rows = board.len();
    let cols = board[0].len();

    // These values are used quite often, so it makes sense to compute them in advance so that
    // we don't need to keep repeating ourselves
    let dim = Dimensions {
        width,
        height,
        rows,
        cols,
        tile_width: width / cols as f64,
        tile_height: height / rows as f64,
    };

    turtle.pen_up();
    turtle.forward(height / 2.0);
    turtle.right(90.0);
    turtle.backward(width / 2.0);
    turtle.pen_down();

    println!("Drawing the board...\n");
    draw_board(&mut turtle, &dim);
    draw_board_pieces(&mut turtle, &board, &dim);
    draw_valid_moves(&mut turtle, &board, &dim);

    // Get rid of any events that may have accumulated while drawing
    drain_events(&mut turtle);

    play_game(&mut turtle, board, &dim);
}

fn draw_board(turtle: &mut Turtle, dim: &Dimensions) {
    turtle.forward(dim.width);
    for i in 0..dim.rows {
        turtle.right((i % 2) as f64 * -180.0 + 90.0);
        turtle.pen_up();
        turtle.forward(dim.height / dim.rows as f64);
        turtle.pen_down();
        turtle.right((i % 2) as f64 * -180.0 + 90.0);
        turtle.forward(dim.width);
    }

    turtle.left(90.0);
    turtle.forward(dim.height);
    for i in 0..dim.cols {
        turtle.left((i % 2) as f64 * -180.0 + 90.0);
        turtle.pen_up();
        turtle.forward(dim.width / dim.cols as f64);
        turtle.pen_down();
        turtle.left((i % 2) as f64 * -180.0 + 90.0);
        turtle.forward(dim.height);
    }
}

fn draw_board_pieces(turtle: &mut Turtle, board: &Board, dim: &Dimensions) {
    // Draw starting pieces
    for (row, row_pieces) in board.iter().enumerate() {
        for (col, piece) in row_pieces.iter().enumerate() {
            if let &Some(piece) = piece {
                move_to_tile(turtle, (row, col), &dim);
                draw_piece(turtle, piece, &dim);
            }
        }
    }
}

fn play_game(turtle: &mut Turtle, mut board: Board, dim: &Dimensions) {
    println!("Click on a tile to make a move.");
    println!("Current Player: {}", board.current().name());
    turtle.set_speed(9);

    let mut mouse = [0.0, 0.0];
    loop {
        let event = turtle.poll_event();
        // Sometimes it is more convenient to use `if let` instead of `match`. In this case, it's
        // really up to your personal preference. We chose to demonstrate what `if let` would look
        // like if used for this code.
        if let Some(Event::MouseMove {x, y}) = event {
            mouse = [x, y];
        }
        else if let Some(Event::MouseButtonReleased(MouseButton::Left)) = event {
            // Figure out which row and column was clicked
            // If these formulas seem unclear, try some example values to see what you get
            let row = ((1.0 - (mouse[1] + dim.height/2.0) / dim.height) * dim.rows as f64).floor() as isize;
            let col = ((mouse[0] + dim.width/2.0) / dim.width * dim.cols as f64).floor() as isize;

            if row >= 0 && row < dim.rows as isize
                && col >= 0 && col < dim.cols as isize
                && board.is_valid_move(&(row as usize, col as usize)) {
                let row = row as usize;
                let col = col as usize;
                erase_valid_moves(turtle, &board, dim);

                board.play_piece((row, col));

                move_to_tile(turtle, (row, col), &dim);
                draw_piece(turtle, board.current(), &dim);
                draw_valid_moves(turtle, &board, dim);

                println!("Current Player: {}", board.current().name());

                // Get rid of any events that may have accumulated while drawing
                drain_events(turtle);
            }
        }
    }
}

/// Moves to the center of the given tile
fn move_to_tile(turtle: &mut Turtle, (row, col): (usize, usize), dim: &Dimensions) {
    let x = col as f64 / dim.cols as f64 * dim.width + dim.tile_width / 2.0 - dim.width / 2.0;
    let y = -(row as f64) / dim.rows as f64 * dim.height - dim.tile_height / 2.0 + dim.height / 2.0;

    turtle.pen_up();

    turtle.turn_towards([x, y]);
    turtle.go_to([x, y]);
    turtle.set_heading(90.0);

    turtle.pen_down();
}

fn erase_valid_moves(turtle: &mut Turtle, board: &Board, dim: &Dimensions) {
    let background = turtle.background_color();
    draw_tile_circles(
        turtle,
        0.5,
        background,
        dim,
        board.valid_moves().iter(),
    );
}

fn draw_valid_moves(turtle: &mut Turtle, board: &Board, dim: &Dimensions) {
    draw_tile_circles(
        turtle,
        0.2,
        board.current().color().with_alpha(0.8),
        dim,
        board.valid_moves().iter(),
    );
}

fn draw_tile_circles<'a, T: Iterator<Item = &'a (usize, usize)>>(
    turtle: &mut Turtle,
    relative_size: f64,
    fill: Color,
    dim: &Dimensions,
    tiles: T,
) {
    let speed = turtle.speed();
    turtle.set_speed("instant");
    for pos in tiles {
        move_to_tile(turtle, *pos, &dim);
        tile_circle(turtle, relative_size, fill, dim);
    }
    turtle.set_speed(speed);
}

/// Draws the given piece
fn draw_piece(turtle: &mut Turtle, piece: Piece, dim: &Dimensions) {
    turtle.show();
    tile_circle(turtle, 0.8, piece.color(), dim);
    turtle.hide();
}

fn tile_circle(turtle: &mut Turtle, relative_size: f64, fill: Color, dim: &Dimensions) {
    let radius = dim.tile_width.min(dim.tile_height) / 2.0 * relative_size;

    filled_circle(turtle, radius, fill);
}

fn filled_circle(turtle: &mut Turtle, radius: f64, fill: Color) {
    turtle.set_fill_color(fill);
    turtle.pen_up();
    turtle.begin_fill();

    turtle.forward(radius);
    turtle.right(90.0);
    circle(turtle, radius);

    turtle.end_fill();
    turtle.pen_down();
}

fn circle(turtle: &mut Turtle, radius: f64) {
    let degrees = 180.0;

    let circumference = 2.0*PI*radius;
    let step = circumference / degrees;
    let rotation = 360.0 / degrees;

    for _ in 0..degrees as i32 {
        turtle.forward(step);
        turtle.right(rotation);
    }
}

/// Clear out all events that may have accumulated
fn drain_events(turtle: &mut Turtle) {
    while let Some(_) = turtle.poll_event() {}
}
