//! Draw a fractal tree.
//! (https://www.youtube.com/watch?v=9UtdjVWSluo)

use turtle::{Color, Point, Turtle};

/// Draw the tree. It's a fractal which consists of 1 large square and 2 small squares.
///
/// `side`: The side of the large square.
/// `depth`: The quantity of the levels of the tree.
fn draw(turtle: &mut Turtle, side: f64, depth: usize) {
    // Draw the large square
    for _ in 0..4 {
        // Get position for setting color
        let position = turtle.position();
        // Each side has a different color
        turtle.set_pen_color(get_color(position));

        turtle.right(90.0);
        turtle.forward(side);
    }

    // If you need to draw a smaller level
    if depth > 0 {
        // The side of the small square
        let smaller_side = side / f64::sqrt(2.0);

        // Move to the starting position of the left small square
        turtle.pen_up();
        turtle.left(45.0);
        turtle.forward(smaller_side);
        let position = turtle.position();
        turtle.pen_down();

        // Remember the heading
        let heading = turtle.heading();

        // Draw the left small square
        // It's the large square on the next level/depth
        draw(turtle, smaller_side, depth - 1);

        // Restore the heading
        turtle.set_heading(heading);

        // Return to the starting position of the left small square
        // Move to the starting position of the right small square
        turtle.pen_up();
        turtle.go_to(position);
        turtle.right(135.0);
        turtle.forward(side);
        turtle.left(45.0);
        turtle.forward(smaller_side);
        turtle.pen_down();

        // Draw the right small square
        // It's the large square on the next level/depth
        draw(turtle, smaller_side, depth - 1);
    }
}

/// Returns the color by the position of the turtle
fn get_color(position: Point) -> Color {
    Color {
        red: 130.0 + f64::abs(position[0] % 75.0),
        green: 150.0 + f64::abs(position[1] % 55.0),
        blue: 210.0 + f64::abs(position[1] % 25.0),
        alpha: 0.8,
    }
}

fn main() {
    let mut turtle = Turtle::new();

    // The side of the first large square
    let side = 100.0;

    // Move to the bottom of the window
    // The tree is growing up, it needs a place above
    turtle.pen_up();
    turtle.backward(1.5 * side);
    turtle.pen_down();

    // Draw the tree
    draw(&mut turtle, side, 6);
}
