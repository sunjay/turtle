//! This example draws star shapes.
//!
//! A star is a circle divided into points, and points are connected across the
//! interior of the circle, instead of to their neighbors.
//!
//! Imagine a five-pointed star. It is made by dividing a circle into five
//! points, and then connecting each point to its neighbors *two* steps away
//! instead of *one*. Connecting each point to its immediate neighbors draws a
//! pentagon, but connecting each point to farther neighbors draws a star.
//!
//! This relationship is true for many different numbers of points, and
//! distances between them. If we divide a circle into some number of points,
//! and pick a number of points around the circle to skip before drawing
//! connecting lines, we can draw all kinds of stars, and see the relationship
//! between the skip count, the point count, and the shape of the produced star.

use turtle::Turtle;

/// The winding number denotes how far around the circumference of the circle to
/// wind an imaginary string, before anchoring it to a point and pulling it
/// tight, as a chord across the interior of the circle.
///
/// Experiment with changing this number to see what shapes the turtle draws!
const WINDING: f64 = 3.0;

/// This divides the circle’s circumference into `n` equal sections.
///
/// Experiment with changing this number to see what shapes the turtle draws!
const POINTS: f64 = 5.0;

/// This computes the angle that the turtle rotates at each point in order to
/// “bounce” off the circle edge and draw the next line.
const TURN: f64 = 360.0 * WINDING / POINTS;

fn main() {
    let mut turtle = Turtle::new();

    turtle.right(90.0);
    turtle.set_pen_size(4.0);
    turtle.set_pen_color("yellow");

    for _ in 0.. POINTS as usize {
        turtle.forward(300.0);
        turtle.right(TURN);
    }
}
