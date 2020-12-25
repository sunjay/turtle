//! View bits in memory with turtles
//!
//! This example uses [@myrrlyn]'s [`bitvec`] crate to turn data into strings of
//! bits, and then draws them on the screen.
//!
//! You are encouraged to change both the data used to seed the turtle, and the
//! `bitvec` calls that control how the turtle acts, to see what changes.
//!
//! [@myrrlyn]: //github.com/myrrlyn
//! [`bitvec`]: //crates.io/crates/bitvec

// This imports the things we need from `bitvec`, including the `Bits` trait for
// the `.view_bits::<_>()` method we use to view memory.
use bitvec::prelude::*;
use turtle::Turtle;

// Modify these constants to change the behavior of the example.

/// This text will be inspected as individual bytes, and drawn on the screen.
/// You can change it to see what different text looks like when viewed as bits.
///
/// The example program will print more information about the parts of the text
/// to the console while the turtle draws, so that you can see how each glyph
/// corresponds to parts of the rendered memory.
static TEXT: &str = "¡Hola, mundo! 🌍🌏🌎";

/// This number will have its bit pattern printed on screen. Rust provides some
/// interesting numbers in its standard library; you can replace this with other
/// numbers to see what they look like. Pi is provided as the default solely
/// because it is well-known, and has an interesting pattern.
const NUMBER: f32 = std::f32::consts::PI;

/// This controls the width of the drawn line for each bit.
const BIT_WIDTH: f64 = 20.0;

/// This controls the vertical spacing between rows of bit lines.
const BIT_HEIGHT: f64 = 10.0;

/// Set the horizontal spacing between successive bits in a row
const BIT_MARGIN: f64 = BIT_WIDTH / 2.0;

/// Compute the total width of a bit plus its spacing
const BIT_BOX: f64 = BIT_WIDTH + BIT_MARGIN;

fn main() {
    // This block sets up the turtle to draw bits more or less centered in the
    // screen. The turtle works by walking horizontally for each bit in a byte,
    // then backtracking and walking vertically to the next byte.
    let mut turtle = Turtle::new();
    // The turtle starts in the center of the screen, but we want to move it
    // around before drawing.
    turtle.pen_up();

    // Compute the boundaries of the part of the screen where the turtle will
    // draw. We expect to be drawing eight bits, with half to the right of
    // center and half to the left.
    let right_edge = BIT_BOX * 8.0 / 2.0;
    // We also expect to be drawing a row for each byte in the text, with an
    // additional separator row for each *character*, half above and half below
    // the center of the screen. This computes how many rows of text we will
    // draw, then moves the turtle appropriately.
    let byte_rows = TEXT.len();
    let char_gaps = TEXT.chars().count();
    let top_edge = BIT_HEIGHT * ((byte_rows + char_gaps) as f64 / 2.0);
    // The turtle starts from the top right of the region,
    turtle.forward(top_edge);
    turtle.right(90.0);
    turtle.forward(right_edge);
    // and walks left
    turtle.left(180.0);

    draw_text(&mut turtle, TEXT);

    // The `draw_number` function reads bits from left to right, so the turtle
    // should also walk from left to right. The `draw_number` function expects
    // that it will be drawing rows sixteen bits long, so it needs to move
    // forward another four bits' worth of space in order to be in the correct
    // spot.
    turtle.forward(8.0 * BIT_BOX / 2.0);
    turtle.forward(16.0 * BIT_BOX / 2.0);
    // Then, it needs to turn around, to walk in the other direction.
    turtle.right(180.0);

    draw_number(&mut turtle, NUMBER);
}

/// Draws the bits of a text span on the screen.
fn draw_text(turtle: &mut Turtle, text: &str) {
    // Rust strings can iterate over their individual characters. This block
    // loops over characters, collecting their start point in the text so that
    // we can grab the encoded bytes of each one.
    let mut row_num = 0;
    for (char_num, (start, codepoint)) in text.char_indices().enumerate() {
        println!("Character {}: {}", char_num, codepoint);
        // Each character has a variable width, so we need to find that.
        let byte_count = codepoint.len_utf8();
        // And then collect the bytes of the string that make up the character.
        // `start` gives us the starting position in the text sequence, and
        // `byte_count` gives us the length in bytes of the character, so we
        // need to select the range beginning at `start`, running for
        // `byte_count`. Another style of writing this that you might see in
        // Rust libraries is `[start ..][.. length]`.
        let row: &[u8] = &text.as_bytes()[start .. start + byte_count];

        // For each byte (`u8`), we use `bitvec` to make a view into its bits.
        // `bitvec` provides the `.view_bits::<_>()` method on Rust integers for
        // easy access to its view types.
        //
        // The `Lsb0` means that the view moves from least significant bit to
        // most significant. Since we want to display on screen the most
        // significant bit on the left, and the least on the right, the turtle
        // will have to move from right to left to match.
        //
        // The `Lsb0` and `Msb0` types describe different ways to view the same
        // data. You can read more about them in the `bitvec` docs, and at
        // Wikipedia:
        // https://docs.rs/bitvec/0.16.1/bitvec/cursor/index.html
        // https://en.wikipedia.org/wiki/Endianness#Bit_endianness
        for byte in row {
            println!("  Byte {:02}:\n    Value: 0x{:02X}\n    Bits: {:08b}", row_num, byte, byte);

            let bits: &BitSlice<_, _> = byte.view_bits::<Lsb0>();

            // Then we draw the byte's bits as a row
            draw_row(turtle, bits);
            //  And go to the next row
            next_row(turtle, 90.0);

            row_num += 1;
        }
        // This puts a dividing line between each *character* in the text.
        // Some characters may have more than one byte, and those bytes will be
        // grouped together.
        delimit(turtle, 8.0 * BIT_BOX - BIT_MARGIN);
    }
}

/// Draws the bits of a number on screen.
fn draw_number(turtle: &mut Turtle, number: f32) {
    // `bitvec` can look at more than just `u8`. Let's try looking at the bits
    // that represent a number!
    //
    // Some numbers, like `f32`, have special rules for their representation in
    // bits. `bitvec` only knows about raw bits, so it does not provide direct
    // support for `f32`. Rust lets us get the bit representation from an `f32`
    // with the method `to_bits(f32) -> u32`, which forgets about the `f32`
    // rules and uses the number's storage as ordinary bits.
    //
    // You can read more about the rules for `f32`'s storage in memory, and
    // behavior in programs, here:
    // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
    let raw_number: u32 = number.to_bits();

    // `bitvec` can also view bits from left to right, with `Msb0`.
    let bits: &BitSlice<_, _> = raw_number.view_bits::<Msb0>();

    // The `&BitSlice` type acts just like `&[bool]`, so it comes with a
    // `.chunks` method which divides it into smaller pieces. `bitvec` can take
    // any number, not just multiples of 8, but 16 is a convenient number to
    // look at. Try changing it to a different number, like 10, to see what
    // happens!
    for (num, row) in bits.chunks(16).enumerate() {
        println!("Row {} bits: {:b}", num, row);
        // Each chunk produced is a smaller `&BitSlice`, just like
        // `&[bool].chunks` produces smaller `&[bool]`s, so we can draw it.
        draw_row(turtle, row);

        next_row(turtle, -90.0);
    }

    // Reader exercise:
    //
    // The IEEE-754 format for `f32` numbers separates them into three parts:
    //
    // 1. The sign marks whether the number is positive or negative: 1 bit
    // 2. The exponent marks how far from zero the number is: 8 bits
    // 3. The fraction describes the number: 23 bits.
    //
    // Using these widths (1 bit, 8 bits, 23 bits), the knowledge that
    // `&BitSlice` is a normal Rust slice, and the API documentation for
    // `std::iter::Iterator`, see if you can display each portion of an `f32`
    // as its own row.
    //
    // Hints:
    //
    // - The variable `bits` is set up to view the entire number, from most
    // significant bit to least.
    // - You can get access to a structure that performs iteration by calling
    //   `bits.iter()`.
    // - You can use the `Iterator::by_ref` method to prevent `Iterator` adapter
    //   functions from destroying the source iterator.
    // - `&BitSlice` is an ordinary Rust slice, so you can use `[start .. end]`
    //   range indexing to get smaller pieces of it.
}

/// Draw a row of bits on the screen.
///
/// This takes a reference to a turtle, which draws, and a reference to a slice
/// of bits, which provides the data to draw.
///
/// Note that this works whether we're going through the bits left to right
/// (`Msb0`) or right to left (`Lsb0`), because we assume that the turtle is
/// going to start on the correct side and be facing the correct way for this
/// drawing to work.
fn draw_row<O, T>(turtle: &mut Turtle, row: &BitSlice<O, T>)
where O: BitOrder, T: BitStore {
    // `&BitSlice` can iterate over bits. It is just like `&[bool]`, and so it
    // produces `&bool` for each loop.
    for bit in row.iter().by_val() {
        // This checks if the bit produced by the row is `1` or `0`, and sets
        // the pen color to black (`1`) or light grey (`0`)
        if bit {
            turtle.set_pen_color("black");
        }
        else {
            turtle.set_pen_color("light grey");
        }

        // For each bit, the loop puts down the pen to draw a line of the bit's
        // color, then picks up the pen to add some horizontal spacing between
        // them.
        turtle.pen_down();
        turtle.forward(BIT_WIDTH);
        turtle.pen_up();
        turtle.forward(BIT_MARGIN);
    }
    //  Rewind the turtle
    for _ in 0 .. row.len() {
        turtle.backward(BIT_BOX);
    }
}

/// Produces a separator line to demark different sections of memory.
fn delimit(turtle: &mut Turtle, width: f64) {
    turtle.set_pen_color("grey");
    turtle.pen_down();
    turtle.forward(width);
    turtle.backward(width);

    next_row(turtle, 90.0);
}

/// Moves the turtle down a row
fn next_row(turtle: &mut Turtle, angle: f64) {
    turtle.pen_up();
    turtle.left(angle);
    turtle.forward(BIT_HEIGHT);
    turtle.right(angle);
}
