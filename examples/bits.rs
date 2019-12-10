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
// the `.bits::<_>()` method we use to view memory.
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
const NUMBER: f64 = std::f64::consts::PI;

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
    turtle.go_to((right_edge, top_edge));
    // and walks left
    turtle.set_heading(180.0);

    // Render the text as bits of memory.

    // Rust strings can iterate over their individual characters. This block
    // loops over characters, collecting their start point in the text so that
    // we can grab the encoded bytes of each one.
    let mut row_num = 0;
    for (char_num, (start, codepoint)) in TEXT.char_indices().enumerate() {
        println!("Character {}: {}", char_num, codepoint);
        // Each character has a variable width, so we need to find that.
        let byte_count = codepoint.len_utf8();
        // And then collect the bytes of the string that make up the character.
        // `start` gives us the starting position in the text sequence, and
        // `byte_count` gives us the length in bytes of the character, so we
        // need to select the range beginning at `start`, running for
        // `byte_count`. Another style of writing this that you might see in
        // Rust libraries is `[start ..][.. length]`.
        let row: &[u8] = &TEXT.as_bytes()[start .. start + byte_count];

        // For each byte (`u8`), we use `bitvec` to make a view into its bits.
        // `bitvec` provides the `.bits::<_>()` method on Rust integers for easy
        // access to its view types.
        //
        // The `LittleEndian` means that the view moves from least significant
        // bit to most significant. Since we want to display on screen the most
        // significant bit on the left, and the least on the right, the turtle
        // will have to move from right to left to match.
        //
        // These types describe different ways to view the same data. You can
        // read more about them in the `bitvec` docs, and at Wikipedia:
        // https://docs.rs/bitvec/0.16.1/bitvec/cursor/index.html
        // https://en.wikipedia.org/wiki/Endianness#Bit_endianness
        for byte in row {
            println!("  Byte {:02}:\n    Value: 0x{:02X}\n    Bits: {:08b}", row_num, byte, byte);

            let bits: &BitSlice<_, _> = byte.bits::<LittleEndian>();

            // Then we draw the byte's bits as a row
            draw_row(&mut turtle, bits);
            // And reset the turtle to the right edge.
            turtle.set_x(right_edge);

            row_num += 1;
        }
        // This puts a dividing line between each *character* in the text.
        // Some characters may have more than one byte, and those bytes will be
        // grouped together.
        delimit(&mut turtle, 8.0 * BIT_BOX - BIT_MARGIN);
    }

    // `bitvec` can look at more than just `u8`. Let's try looking at the bits
    // that represent a number!
    //
    // Some numbers, like `f64`, have special rules for their representation in
    // bits. `bitvec` only knows about raw bits, so it does not provide direct
    // support for `f64`. Rust lets us get the bit representation from an `f64`
    // with the method `to_bits(f64) -> u64`, which forgets about the `f64`
    // rules and uses the number's storage as ordinary bits.
    //
    // You can read more about the rules for `f64`'s storage in memory, and
    // behavior in programs, here:
    // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
    let raw_number: u64 = NUMBER.to_bits();

    // `bitvec` can also view bits from left to right, with `BigEndian`.
    let bits: &BitSlice<_, _> = raw_number.bits::<BigEndian>();

    // Since we are reading bits from left to right, the turtle should move from
    // left to right also. You can change the `* 2.0` to move the section
    // horizontally.
    let left_edge = -right_edge * 2.0;
    turtle.set_x(left_edge);
    // Walk from left to right
    turtle.set_heading(0.0);

    // The `&BitSlice` type acts just like `&[bool]`, so it comes with a
    // `.chunks` method which divides it into smaller pieces. `bitvec` can take
    // any number, not just multiples of 8, but 16 is a convenient number to
    // look at. Try changing it to a different number, like 10, to see what
    // happens!
    for (num, row) in bits.chunks(16).enumerate() {
        println!("Row {} bits: {:b}", num, row);
        // Each chunk produced is a smaller `&BitSlice`, just like
        // `&[bool].chunks` produces smaller `&[bool]`s, so we can draw it.
        draw_row(&mut turtle, row);
        // After each row, the turtle has to go back to the left edge.
        turtle.set_x(left_edge);
    }

    // Reader exercise:
    //
    // The IEEE-754 format for `f64` numbers separates them into three parts:
    //
    // 1. The sign marks whether the number is positive or negative: 1 bit
    // 2. The exponent marks how far from zero the number is: 11 bits
    // 3. The fraction describes the number: 52 bits.
    //
    // Using these widths (1 bit, 11 bits, 52 bits), the knowledge that
    // `&BitSlice` is a normal Rust slice, and the API documentation for
    // `std::iter::Iterator`, see if you can display each portion of an `f64`
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
/// (`BigEndian`) or right to left (`LittleEndian`), because we assume that the
/// turtle is going to start on the correct side and be facing the correct way
/// for this drawing to work.
fn draw_row<C, T>(t: &mut Turtle, row: &BitSlice<C, T>)
where C: Cursor, T: BitStore {
    // `&BitSlice` can iterate over bits. It is just like `&[bool]`, and so it
    // produces `&bool` for each loop.
    for &bit in row {
        // This checks if the bit produced by the row is `1` or `0`, and sets
        // the pen color to black (`1`) or light grey (`0`)
        if bit {
            t.set_pen_color("black");
        }
        else {
            t.set_pen_color("light grey");
        }

        // For each bit, the loop puts down the pen to draw a line of the bit's
        // color, then picks up the pen to add some horizontal spacing between
        // them.
        t.pen_down();
        t.forward(BIT_WIDTH);
        t.pen_up();
        t.forward(BIT_MARGIN);
    }

    next_row(t, BIT_HEIGHT);
}

/// Produces a separator line to demark different sections of memory.
fn delimit(t: &mut Turtle, width: f64) {
    t.set_pen_color("grey");
    t.pen_down();
    t.forward(width);
    t.backward(width);

    next_row(t, BIT_HEIGHT);
}

/// Moves the turtle down a row
fn next_row(t: &mut Turtle, by: f64) {
    // To move down, the turtle picks up its pen,
    t.pen_up();
    // rememebers which direction it was going,
    let old_heading = t.heading();
    // turns to face down the screen,
    t.set_heading(270.0);
    // moves down by a row,
    t.forward(by);
    // then goes back to its old direction.
    t.set_heading(old_heading);
    // This way each row gets vertical spacing between them.
}
