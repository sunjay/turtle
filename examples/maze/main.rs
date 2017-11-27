mod wall;
mod cell;
mod maze;
mod grid;
mod solver;

extern crate turtle;

use turtle::{self, Turtle, Color};

pub use maze::Maze;
use grid::{GridCellIter};
use solver::solve;

// Dimensions of the maze in turtle steps (pixels)
const WIDTH: f64 = 600.0; // px
const HEIGHT: f64 = 600.0; // px

fn main() {
    turtle::start();

    let maze = Maze::generate();

    let mut turtle = Turtle::new();
    turtle.set_speed(8);
    turtle.set_background_color("#BDBDBD");
    turtle.set_pen_color("#03A9F4");
    turtle.set_pen_size(2.0);

    // Get to the top left corner
    turtle.pen_up();
    turtle.forward(HEIGHT / 2.0);
    turtle.right(90.0);
    turtle.backward(WIDTH / 2.0);
    turtle.pen_down();

    let cell_width = WIDTH/(maze.row_size() as f64);
    let cell_height = HEIGHT/(maze.col_size() as f64);

    // Draw rows
    draw_rows(
        &mut turtle,
        cell_width,
        cell_height,
        maze.first_row().map(|cell| cell.north.is_closed()),
        maze.rows(),
        |row| row.map(|cell| cell.south.is_closed()),
        false,
    );

    // Draw columns
    turtle.left(90.0);
    draw_rows(
        &mut turtle,
        cell_height,
        cell_width,
        maze.last_col().rev().map(|cell| cell.east.is_closed()),
        maze.cols().rev(),
        |col| col.map(|cell| cell.west.is_closed()).rev(),
        true,
    );

    turtle.right(90.0);
    draw_marker(&mut turtle, maze.start(), cell_width, cell_height, "#C5E1A5");
    draw_marker(&mut turtle, maze.finish(), cell_width, cell_height, "#FFAB91");

    turtle.pen_up();
    turtle.forward(cell_width * 0.5);
    turtle.right(90.0);
    turtle.forward(cell_height * 0.5);
    turtle.left(90.0);
    turtle.pen_down();

    turtle.set_speed(5);
    turtle.set_pen_size(2.0);
    solve(&mut turtle, maze, cell_width, cell_height);
}

fn draw_rows<
    'a,
    R: Iterator<Item=bool>,
    G: Iterator<Item=GridCellIter<'a>>,
    F: Fn(GridCellIter<'a>) -> X,
    X: Iterator<Item=bool> + DoubleEndedIterator
>(
    turtle: &mut Turtle,
    // size of each cell in the row
    cell_size: f64,
    // gap between rows
    cell_gap: f64,
    first_row: R,
    rows: G,
    row_walls: F,
    rotate_left: bool,
) {
    draw_row(turtle, cell_size, first_row);

    // Direction of rotation for all turns
    let rotation = if rotate_left { -1.0 } else { 1.0 };
    for (i, row) in rows.enumerate() {
        turtle.pen_up();

        // Direction of rotation for these turns
        let direction = rotation * if i % 2 == 0 { 1.0 } else { -1.0 };
        turtle.right(direction * 90.0);
        turtle.forward(cell_gap);
        turtle.right(direction * 90.0);
        turtle.pen_down();

        let walls = row_walls(row);

        // Every second row needs to be reversed so the turtle can zig-zag back and
        // forth instead of wasting too much time moving all the way to the left of
        // each row
        if i % 2 == 0 {
            draw_row(turtle, cell_size, walls.rev());
        }
        else {
            draw_row(turtle, cell_size, walls);
        }
    }
}

fn draw_row<I: Iterator<Item=bool>>(turtle: &mut Turtle, wall_size: f64, walls: I) {
    for should_draw in walls {
        if !should_draw {
            turtle.pen_up();
        }
        turtle.forward(wall_size);
        turtle.pen_down();
    }
}

fn draw_marker<C: Into<Color>>(
    turtle: &mut Turtle,
    (row, col): (usize, usize),
    cell_width: f64,
    cell_height: f64,
    color: C,
) {
    turtle.pen_up();

    let row = row as f64;
    let col = col as f64;
    let speed = turtle.speed();

    turtle.set_speed("instant");
    turtle.forward(col * cell_width);
    turtle.right(90.0);
    turtle.forward(row * cell_height);
    turtle.left(90.0);
    turtle.set_speed(speed);

    // The padding around the start and finish markers
    let marker_padding = turtle.pen_size(); // px

    turtle.set_fill_color(color);
    turtle.forward(marker_padding);
    turtle.right(90.0);
    turtle.forward(marker_padding);
    turtle.left(90.0);

    turtle.begin_fill();
    for _ in 0..2 {
        turtle.forward(cell_width - marker_padding * 2.0);
        turtle.right(90.0);
        turtle.forward(cell_height - marker_padding * 2.0);
        turtle.right(90.0);
    }
    turtle.end_fill();

    turtle.backward(marker_padding);
    turtle.right(90.0);
    turtle.backward(marker_padding);
    turtle.left(90.0);

    turtle.set_speed("instant");
    turtle.backward(col * cell_width);
    turtle.right(90.0);
    turtle.backward(row * cell_height);
    turtle.left(90.0);
    turtle.set_speed(speed);

    turtle.pen_down();
}
