use std::collections::{VecDeque, HashSet};

use turtle::{Turtle, thread_rng, Rng};
use maze::Maze;

const SOLUTION_COLOR: &str = "#4CAF50";
const BACKTRACK_COLOR: &str = "#F44336";

/// Visually solve the maze by physically moving the turtle through it
pub fn solve(turtle: &mut Turtle, maze: Maze, cell_width: f64, cell_height: f64) {
    turtle.set_pen_color(SOLUTION_COLOR);

    let mut visited = HashSet::<(usize, usize)>::new();
    // Keeps track of where we move as we go there
    // Positions are removed as we backtrack
    let mut path_stack = VecDeque::<(usize, usize)>::new();
    let mut current = (0, 0);

    let mut rng = thread_rng();
    loop {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        if current == maze.finish() {
            break;
        }

        // We force the compiler to copy here so that we do not get a borrow checker error since
        // current is mutated below
        let curr = current;
        let mut unvisited = maze.adjacent_cells(current).into_iter()
            .filter(|p| maze.is_open_between(curr, *p))
            .filter(|p| !visited.contains(p))
            .peekable();

        if unvisited.peek().is_none() {
            // Dead end, start backtracking until we have unvisited adjacents
            loop {
                let previous = match path_stack.pop_back() {
                    Some(pos) => pos,
                    None => unreachable!("Backtracked to the beginning. Could not find solution for maze!"),
                };
                // Animate to previous
                turtle.set_pen_color(BACKTRACK_COLOR);

                // If previous has unvisited adjacents, make it the current and break this loop
                // Otherwise keep going
            }
        }
        else {
            path_stack.push_back(current);

            let mut unvisited = unvisited.collect::<Vec<_>>();
            rng.shuffle(&mut unvisited);

            let next = *unvisited.first().unwrap();
            move_to(turtle, current, next, cell_width, cell_height);
            current = next;
        }
    }
}

fn move_to(
    turtle: &mut Turtle,
    (curr_row, curr_col): (usize, usize),
    (next_row, next_col): (usize, usize),
    cell_width: f64,
    cell_height: f64,
) {
    let delta = (next_row as isize - curr_row as isize, next_col as isize - curr_col as isize);
    let (target_heading, distance) = match delta {
        // go north
        (-1, 0) => (90.0, cell_height),
        // go east
        (0, 1) => (0.0, cell_width),
        // go south
        (1, 0) => (270.0, cell_height),
        // go west
        (0, -1) => (180.0, cell_width),
        _ => unreachable!("Attempt to move to non-adjacent cell"),
    };

    let heading = turtle.heading();
    // Find the amount we need to turn to reach the target heading based on our current heading
    let angle = target_heading - heading;
    // Normalize the angle to be between -180 and 179 so that we rotate as little as possible
    // Formula from: https://stackoverflow.com/a/24234924/551904
    let angle = angle - 360.0 * ((angle + 180.0) / 360.0).floor();

    turtle.left(angle);
    turtle.forward(distance);
}
