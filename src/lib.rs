use std::env;

enum OutputMode {
    /// Actually spawn a window and draw the turtle.
    /// For when the program is running from a terminal.
    Draw,
    /// Only print the drawing commands, but don't spawn
    /// any windows or anything.
    /// For when the program is running in the Turtle IDE
    Print,
}

/// A turtle with a pen attached to its tail
pub struct Turtle {
    mode: OutputMode,
}

impl Turtle {
    /// Initialize a new Turtle instance
    fn new() -> Turtle {
        // Attempt to automatically detect if this is running within the Turtle IDE
        let mode = match env::args().any(|a| a == "--turtle-ide-print-mode") {
            false => OutputMode::Draw,
            true => OutputMode::Print,
        };

        Turtle {mode}
    }

    /// Move the turtle forward by the given amount of distance
    ///
    /// distance is given in "pixels" which are like really small turtle steps
    fn forward(distance: i32) {

    }
}
