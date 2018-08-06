use std::collections::{HashSet, VecDeque};

use maze::Maze;
use turtle::rand::{thread_rng, Rng};
use turtle::Turtle;

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
        visited.insert(current);

        if current == maze.finish() {
            break;
        }

        let mut unvisited = unvisited_open_adjacents(&maze, &visited, current);
        rng.shuffle(&mut unvisited);

        if unvisited.is_empty() {
            // Dead end, start backtracking until we have unvisited adjacents
            turtle.set_pen_color(BACKTRACK_COLOR);
            loop {
                let previous = match path_stack.pop_back() {
                    Some(pos) => pos,
                    None => unreachable!("Backtracked to the beginning. Could not find solution for maze!"),
                };

                move_to(turtle, current, previous, cell_width, cell_height);
                current = previous;

                let unvisited = unvisited_open_adjacents(&maze, &visited, previous);
                if !unvisited.is_empty() {
                    break;
                }
            }
            turtle.set_pen_color(SOLUTION_COLOR);
        } else {
            path_stack.push_back(current);

            let next = *unvisited.first().unwrap();
            move_to(turtle, current, next, cell_width, cell_height);
            current = next;
        }
    }
}

fn unvisited_open_adjacents(maze: &Maze, visited: &HashSet<(usize, usize)>, position: (usize, usize)) -> Vec<(usize, usize)> {
    maze.adjacent_cells(position)
        .into_iter()
        .filter(|p| maze.is_open_between(position, *p))
        .filter(|p| !visited.contains(p))
        .collect()
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
