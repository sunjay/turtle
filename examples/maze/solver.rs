use std::collections::{VecDeque, HashSet};

use turtle::{Turtle, thread_rng, Rng};
use maze::Maze;

const SOLUTION_COLOR: &str = "#4CAF50";
const BACKTRACK_COLOR: &str = "#F44336";

/// Visually solve the maze by physically moving the turtle through it
pub fn solve(turtle: &mut Turtle, maze: Maze, cell_width: f64, cell_height: f64) {
    turtle.set_pen_size(2.0);
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

        let adjacents = maze.adjacent_cells(current);
        let mut unvisited = adjacents.into_iter()
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
            // Animate to next
            current = next;
        }
    }
}
