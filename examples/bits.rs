//! View bits in memory with turtles
//!
//! This example uses [@myrrlyn]’s [`bitvec`] crate to turn data into strings of
//! bits, and then draws them on the screen.
//!
//! You are encouraged to change both the data used to seed the turtle, and the
//! `bitvec` calls that control how the turtle acts, to see what changes.
//!
//! [@myrrlyn]: //github.com/myrrlyn
//! [`bitvec`]: //crates.io/crates/bitvec

// The `bitvec` crate provides a bunch of types for us to use with one import
use bitvec::prelude::*;
use turtle::Turtle;

/// Set the line length for each displayed bit
const BIT_WIDTH: f64 = 20.0;
/// Set the vertical spacing between successive rows of bits
const BIT_HEIGHT: f64 = 10.0;
/// Set the horizontal spacing between successive bits in a row
const BIT_MARGIN: f64 = BIT_WIDTH / 2.0;
/// Compute the total width of a bit plus its spacing
const BIT_BOX: f64 = BIT_WIDTH + BIT_MARGIN;

fn main() {
    // Replace this text with a message of your own to see how it changes on the
    // screen.
    let text = "Hello, world!";

    // This block sets up the turtle to draw bits more or less centered in the
    // screen. The turtle works by walking horizontally for each bit in a byte,
    // then backtracking and walking vertically to the next byte.
    let mut turtle = Turtle::new();
    turtle.pen_up();
    // Compute the boundaries of the part of the screen where the turtle will
    // draw
    let right_edge = BIT_BOX * 4.0;
    let top_edge = BIT_HEIGHT * (text.len() as f64 / 2.0);
    // The turtle starts from the top right of the region,
    turtle.go_to((right_edge, top_edge));
    // and walks left
    turtle.set_heading(180.0);

    // Now let’s draw some data on the screen, as individual bits

    // Rust strings can iterate over the bytes used to represent them.
    for byte in text.bytes() {
        // For each byte (`u8`), we use `bitvec` to make a view into its bits.
        //
        // The `LittleEndian` means that the view moves from right to left
        // across the byte, which fits how our turtle moves from right to left
        // across the screen.
        let bits = byte.bits::<LittleEndian>();
        // Then we draw the byte’s bits as a row
        draw_row(&mut turtle, bits);
        // And reset the turtle to the right edge.
        turtle.set_x(right_edge);
    }

    // `bitvec` can look at more than just `u8`. Let’s try looking at the bits
    // that represent a number!

    // First we need to pick a number. Rust provides some interesting numbers
    // for us.
    let tau = std::f64::consts::PI * 2.0;
    // But `bitvec` doesn’t know how to view all numbers. The standard library
    // provides a function `to_bits(f64) -> u64`, which turns it into a number
    // `bitvec` does know how to view.
    let raw_number = tau.to_bits();
    // `bitvec` can also view bits from left to right, with `BigEndian`.
    let bits = raw_number.bits::<BigEndian>();

    // Since we are reading bits from left to right, the turtle should move from
    // left to right also. Change the `* 2.0` to move the section horizontally.
    let left_edge = -right_edge * 2.0;
    turtle.set_x(left_edge);
    // Walk from left to right
    turtle.set_heading(0.0);

    // The `&BitSlice` type acts just like `&[bool]`, so it comes with a
    // `.chunks` method which divides it into smaller pieces. `bitvec` can take
    // any number, not just multiples of 8, but 16 is a convenient number to
    // look at. Try changing it to a different number, like 10, to see what
    // happens!
    for row in bits.chunks(16) {
        // Each chunk produced is a smaller `&BitSlice`, just like
        // `&[bool].chunks` produces smaller `&[bool]`s, so we can draw it.
        draw_row(&mut turtle, row);
        // After each row, the turtle has to go back to the left edge.
        turtle.set_x(left_edge);
    }
}

/// Draw a row of bits on the screen.
///
/// This takes a reference to a turtle, which draws, and a reference to a slice
/// of bits, which provides the data to draw.
fn draw_row<C, T>(t: &mut Turtle, row: &BitSlice<C, T>)
where C: Cursor, T: BitStore {
    // `&BitSlice` can iterate over bits. It is just like `&[bool]`, and so it
    // produces `&bool` for each loop.
    for bit in row {
        // This checks if the bit produced by the row is `1` or `0`, and sets
        // the pen color to black (`1`) or light grey (`0`)
        if *bit {
            t.set_pen_color("black");
        }
        else {
            t.set_pen_color("light grey");
        }

        // For each bit, the loop puts down the pen to draw a line of the bit’s
        // color, then picks up the pen to add some horizontal spacing between
        // them.
        t.pen_down();
        t.forward(BIT_WIDTH);
        t.pen_up();
        t.forward(BIT_MARGIN);
    }

    // After the row is complete, the turtle picks up its pen,
    t.pen_up();
    // rememebers which direction it was going,
    let old_heading = t.heading();
    // turns to face down the screen,
    t.set_heading(270.0);
    // moves down by a row,
    t.forward(BIT_HEIGHT);
    // then goes back to its old direction.
    t.set_heading(old_heading);
    // This way each row gets vertical spacing between them.
}
