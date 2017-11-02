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

// Dimensions of the maze in turtle steps (pixels)
const WIDTH: usize = 600; // px
const HEIGHT: usize = 600; // px

fn main() {
    let maze = Maze::new();

    let mut turtle = Turtle::new();
    turtle.set_speed(9);
    turtle.set_background_color("#A5D6A7");
    turtle.set_pen_color("#03A9F4");
    turtle.set_pen_size(5.0);

    // Get to the top left corner
    turtle.pen_up();
    turtle.forward((HEIGHT / 2) as f64);
    turtle.right(90.0);
    turtle.backward((WIDTH / 2) as f64);
    turtle.pen_down();

    // Draw rows
    let cell_width = (WIDTH as f64)/(maze.row_size() as f64);
    let cell_height = (WIDTH as f64)/(maze.row_size() as f64);

    // Draw first row
    draw_row(&mut turtle, cell_width,
        maze.first_row().iter().map(|cell| cell.north.is_closed()));

    for (i, row) in maze.rows().enumerate() {
        turtle.pen_up();
        let direction = if i % 2 == 0 { 1.0 } else { -1.0 };
        turtle.right(direction * 90.0);
        turtle.forward(cell_height);
        turtle.right(direction * 90.0);
        turtle.pen_down();

        let walls = row.map(|cell| cell.south.is_closed());

        // Every second row needs to be reversed so the turtle can zig-zag back and
        // forth instead of wasting too much time moving all the way to the left of
        // each row
        if i % 2 == 0 {
            draw_row(&mut turtle, cell_width, walls);
        } else {
            draw_row(&mut turtle, cell_width, walls.rev());
        }
    }

    // Draw columns
    turtle.left(90.0);

    // Draw first column
    draw_row(&mut turtle, cell_width,
        maze.last_col().iter().map(|cell| cell.west.is_closed()));

    for (i, row) in maze.cols().rev().enumerate() {
        turtle.pen_up();
        let direction = if i % 2 == 0 { -1.0 } else { 1.0 };
        turtle.right(direction * 90.0);
        turtle.forward(cell_height);
        turtle.right(direction * 90.0);
        turtle.pen_down();

        let walls = row.map(|cell| cell.east.is_closed());

        // Every second row needs to be reversed so the turtle can zig-zag back and
        // forth instead of wasting too much time moving all the way to the left of
        // each row
        if i % 2 == 0 {
            draw_row(&mut turtle, cell_width, walls);
        } else {
            draw_row(&mut turtle, cell_width, walls.rev());
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
