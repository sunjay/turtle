//! Draws a Sierpiński triangle with automatic positioning and sizing.
//!
//! The Sierpiński triangle is a fairly simple self-similar fractal geometric shape: it consists of
//! many nested equilateral triangles. More formally, such a triangle is itself three triangles of
//! one level below and a size divided by two. Level zero means a simple equilateral triangle. The
//! drawing procedure is as follows, for a given level and size:
//!
//!  * If level is 0
//!    * Draw an equilateral triangle of the given size.
//!  * otherwise
//!    * Draw the half-sized level - 1 triangle at the bottom left.
//!    * Go the start of the bottom-right slot.
//!    * Draw a half-sized level - 1 triangle.
//!    * Go to the start of the top slot.
//!    * Draw a half-sized level - 1 triangle.
//!
//! That is relatively easy to implement, as long as you follow these steps and let recursion do
//! the rest. Another little bonus this example provides is the ability to customize the drawing
//! size: the triangle will stay correctly sized and positioned automatically.

use turtle::{Distance, Drawing, Point, Size, Turtle};

/// The number of levels to draw following the recursive procedure.
const LEVELS: u8 = 6;
/// The turtle speed to use.
const SPEED: f64 = 25.0;
/// The margin expressed in pixels that will be saved from all sides when auto-sizing.
const BORDER_MARGIN: Distance = 15.0;
/// Optional parameter setting the canvas size to use.
///
/// `None` means using the default size. Provide `Some(Size)` in order to specify a custom one.
const CANVAS_SIZE: Option<Size> = None;

/// Convenience function grouping the operations of turning the `turtle` towards and go to the
/// given `point`.
fn turn_and_go_to(turtle: &mut Turtle, dest: Point) {
    turtle.turn_towards(dest);
    turtle.go_to(dest);
}

/// Recursive function drawing a Sierpiński triangle.
///
/// It will do it with the given `turtle` and start at its current position and heading. `level`
/// is the depth of the drawing to be done, zero meaning a simple triangle. `size` is the length
/// of the outermost triangle's sides.
fn sierpinski_triangle(turtle: &mut Turtle, level: u8, size: Distance) {
    // When level 0 is reached, just draw an equilateral triangle.
    if level == 0 {
        turtle.pen_down();

        for _ in 0..3 {
            turtle.forward(size);
            turtle.left(120.0);
        }

        turtle.pen_up();
    } else {
        // Parameters for subsequent calls are the same.
        let next_level = level - 1;
        let next_size = size / 2.0;

        // Bottom-left triangle.
        sierpinski_triangle(turtle, next_level, next_size);

        turtle.forward(next_size);

        // Bottom-right triangle.
        sierpinski_triangle(turtle, next_level, next_size);

        turtle.left(120.0);
        turtle.forward(next_size);
        turtle.right(120.0);

        // Top triangle.
        sierpinski_triangle(turtle, next_level, next_size);

        // Go back to the start.
        turtle.right(120.0);
        turtle.forward(next_size);
        turtle.left(120.0);
    }
}

/// Draws a Sierpiński triangle with automatic size and start point.
///
/// `level` is still required, it can't be computed automatically. However, given the used
/// `canvas_size`, it will compute the appropriate size and start point so the triangle gets
/// centered and occupies as much drawing space as possible while staying in bounds.
fn sierpinski_triangle_auto(turtle: &mut Turtle, level: u8, canvas_size: Size) {
    // The maximum size the triangle can take without going over the margin.
    let auto_size = (canvas_size.width as f64).min(canvas_size.height as f64 * 2.0 / 3f64.sqrt())
        - 2.0 * BORDER_MARGIN;

    turtle.pen_up();
    turn_and_go_to(
        turtle,
        [-auto_size / 2.0, -auto_size / 4.0 * 3f64.sqrt()].into(),
    );
    turtle.set_heading(0.0); // 0 = East.

    // The drawing itself.
    sierpinski_triangle(turtle, level, auto_size);
}

/// Example's main function: sets up the drawing, a turtle and calls the procedure.
fn main() {
    let mut drawing = Drawing::new();

    // Set drawing size when not `None` in order to let the default value be available.
    if let Some(canvas_size) = CANVAS_SIZE {
        drawing.set_size(canvas_size);
    }

    let mut turtle = drawing.add_turtle();
    turtle.use_degrees();
    turtle.set_speed(SPEED);

    // Auto-sized procedure.
    sierpinski_triangle_auto(&mut turtle, LEVELS, drawing.size());

    // Hide turtle when done drawing in order to fully reveal the result.
    turtle.hide();
}
