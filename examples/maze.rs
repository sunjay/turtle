//HACK: This is how we're splitting the maze example into multiple files since
// cargo doesn't properly support examples with multiple files yet
#[path = "maze/wall.rs"]
mod wall;
#[path = "maze/cell.rs"]
mod cell;
#[path = "maze/maze.rs"]
mod maze;
#[path = "maze/grid.rs"]
mod grid;

extern crate turtle;

use turtle::Turtle;

pub use maze::Maze;
use grid::{GridCellIter};
use cell::Cell;

// Dimensions of the maze in turtle steps (pixels)
const WIDTH: f64 = 600.0; // px
const HEIGHT: f64 = 600.0; // px

fn main() {
    let maze = Maze::generate();

    let mut turtle = Turtle::new();
    turtle.set_speed(9);
    turtle.set_background_color("#A5D6A7");
    turtle.set_pen_color("#03A9F4");
    turtle.set_pen_size(5.0);

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
        |cell| cell.south.is_closed(),
        false,
    );

    // Draw columns
    turtle.left(90.0);
    draw_rows(
        &mut turtle,
        cell_height,
        cell_width,
        maze.last_col().map(|cell| cell.west.is_closed()),
        maze.cols().rev(),
        |cell| cell.east.is_closed(),
        true,
    );
}

fn draw_rows<'a, R: Iterator<Item=bool>, G: Iterator<Item=GridCellIter<'a>>, F: Fn(&Cell) -> bool>(
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
        let direction = rotation * if i % 2 == 0 {
            1.0
        } else {
            -1.0
        };
        turtle.right(direction * 90.0);
        turtle.forward(cell_gap);
        turtle.right(direction * 90.0);
        turtle.pen_down();

        let walls = row.map(&row_walls);

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
