//! Draws a simple geometric sort of flower with customizable dimensions.
//!
//! This example makes extensive use of the turtle arc methods: [`arc_left`] and [`arc_right`].
//! They both provide the ability to draw a circular arc defined by the given `radius` and `extent`
//! parameters, while the first makes the turtle draw it to the left and the second to the right.

// To run this example, use the command: cargo run --features unstable --example flower
#[cfg(all(not(feature = "unstable")))]
compile_error!("This example relies on unstable features. Run with `--features unstable`");

use turtle::{Angle, Distance, Drawing};

const TURTLE_SPEED: &str = "faster";
const BOTTOM_MARGIN: Distance = 25.0;

const LEAF_FILL_COLOR: &str = "green";
const LEAF_BORDER_COLOR: &str = "dark green";
const LEAF_BORDER_WIDTH: Distance = 1.0;
const LEFT_LEAF_RADIUS: Distance = 200.0;
const LEFT_LEAF_EXTENT: Angle = 45.0;
const RIGHT_LEAF_INCLINATION: Angle = 15.0;
const RIGHT_LEAF_BOTTOM_RADIUS: Distance = 250.0;
const RIGHT_LEAF_BOTTOM_EXTENT: Angle = 45.0;
const RIGHT_LEAF_TOP_RADIUS: Distance = 157.0;
const RIGHT_LEAF_TOP_EXTENT: Angle = 75.0;

const TRUNK_COLOR: &str = LEAF_BORDER_COLOR;
const TRUNK_WIDTH: Distance = 3.0;
const TRUNK_PIECE_COUNT: usize = 4;
const TRUNK_PIECE_RADIUS: Distance = 500.0;
const TRUNK_PIECE_EXTENT: Angle = 15.0;

const PETALS_COUNT: usize = 4;
const PETALS_FILL_COLOR: &str = "purple";
const PETALS_BORDER_COLOR: &str = "dark purple";
const PETALS_BORDER_WIDTH: Distance = LEAF_BORDER_WIDTH;
const PETALS_INIT_LEFT: Angle = 65.0;
const PETALS_SIDE_RADIUS: Distance = 80.0;
const PETALS_SIDE_EXTENT: Angle = 90.0;
const PETALS_SPACE_GAP: Angle = 20.0;
const PETALS_SPACE_RADIUS: Distance = 40.0;
const PETALS_SPACE_EXTENT: Angle = 30.0;

fn main() {
    // Acquiring resources.
    let mut drawing = Drawing::new();
    let size = drawing.size();
    let mut turtle = drawing.add_turtle();

    // Initial positioning.
    turtle.set_speed("instant");
    turtle.pen_up();
    turtle.go_to([
        -(size.width as f64) / 6.0,
        -(size.height as f64) / 2.0 + BOTTOM_MARGIN,
    ]);
    turtle.pen_down();

    // Setup.
    turtle.use_degrees();
    turtle.set_speed(TURTLE_SPEED);

    // Body.
    turtle.set_fill_color(LEAF_FILL_COLOR);
    turtle.set_pen_color(LEAF_BORDER_COLOR);

    for _ in 0..TRUNK_PIECE_COUNT {
        // Leaves:
        turtle.set_pen_size(LEAF_BORDER_WIDTH);
        turtle.set_pen_color(LEAF_BORDER_COLOR);
        turtle.begin_fill();

        // Left leaf.
        turtle.arc_left(LEFT_LEAF_RADIUS, LEFT_LEAF_EXTENT);
        turtle.right(LEFT_LEAF_EXTENT);
        turtle.arc_right(LEFT_LEAF_RADIUS, -LEFT_LEAF_EXTENT);
        turtle.right(LEFT_LEAF_EXTENT);

        // Right leaf.
        turtle.right(RIGHT_LEAF_INCLINATION);
        // Note that `arc_left` with a negative radius is the same as calling `arc_right`.
        // This is used below for illustration purposes only. You'd probably want to use
        // `arc_right` in real code.
        turtle.arc_left(-RIGHT_LEAF_BOTTOM_RADIUS, RIGHT_LEAF_BOTTOM_EXTENT);
        turtle.right(RIGHT_LEAF_INCLINATION);
        turtle.arc_left(-RIGHT_LEAF_TOP_RADIUS, -RIGHT_LEAF_TOP_EXTENT);

        // Trunk.
        turtle.end_fill();
        turtle.set_pen_size(TRUNK_WIDTH);
        turtle.set_pen_color(TRUNK_COLOR);
        turtle.arc_right(TRUNK_PIECE_RADIUS, TRUNK_PIECE_EXTENT);
    }

    // Petals.
    turtle.set_fill_color(PETALS_FILL_COLOR);
    turtle.set_pen_color(PETALS_BORDER_COLOR);
    turtle.set_pen_size(PETALS_BORDER_WIDTH);
    turtle.left(PETALS_INIT_LEFT);
    turtle.begin_fill();
    turtle.arc_right(PETALS_SIDE_RADIUS, PETALS_SIDE_EXTENT);

    for _ in 0..PETALS_COUNT {
        turtle.left(PETALS_SPACE_GAP);
        turtle.arc_right(PETALS_SPACE_RADIUS, -PETALS_SPACE_EXTENT);
        turtle.right(2.0 * PETALS_SPACE_GAP + PETALS_SPACE_EXTENT);
        turtle.arc_left(PETALS_SPACE_RADIUS, PETALS_SPACE_EXTENT);
    }

    // Finish petals with error adjustments.
    turtle.left(PETALS_SPACE_GAP);
    turtle.arc_left(PETALS_SIDE_RADIUS + 1.0, 3.0 - PETALS_SIDE_EXTENT);
    turtle.end_fill();

    // Reveal final drawing.
    turtle.hide();
}
