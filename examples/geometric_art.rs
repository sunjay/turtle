use std::iter::from_fn;

use turtle::{rand::{choose, random}, Color, Drawing, Turtle};

fn main() {
    // Parameters to play around with for changing the character of the created drawing.
    let width = 800;
    let height = 600;
    let row_count = 3;
    let col_count = 4;
    let color_count = 7;

    let mut turtle = ArtisticTurtle::new(width, height, row_count, col_count, color_count);
    turtle.draw();
}

/// A special kind of turtle that is specialized in creating drawings of colorful triangles.
struct ArtisticTurtle {
    turtle: Turtle,
    row_count: u32,
    col_count: u32,
    row_height: f64,
    col_width: f64,
    colors: Vec<Color>,
}

impl ArtisticTurtle {
    /// Create a new turtle using the given parameters.
    /// `width`: total width of the drawing
    /// `height`: total height of the drawing
    /// `row_count`: number of rows of triangles to be created
    /// `col_count`: number of columns per row to be created
    /// `color_count`: number of different colors to use
    fn new(width: u32, height: u32, row_count: u32, col_count: u32, color_count: usize) -> Self {
        // Create and prepare the turtle.
        let mut drawing = Drawing::new();
        let mut turtle = drawing.add_turtle();
        turtle.set_speed("instant");
        turtle.pen_up();
        drawing.set_size([width, height]);

        // Move turtle to the upper left corner of the drawing.
        let size = drawing.size();
        let x_start = size.width / 2;
        let y_start = size.height / 2;
        turtle.go_to([f64::from(x_start) * -1.0, f64::from(y_start)]);
        turtle.set_speed(20);

        // Calculate height and width of the triangles.
        let row_height = f64::from(size.height) / f64::from(row_count);
        let col_width = f64::from(size.width) / f64::from(col_count);

        // Prepare a set of random colors to randomly choose from when drawing.
        let colors: Vec<Color> = from_fn(|| Some(random())).take(color_count).collect();

        Self {
            turtle,
            row_count,
            col_count,
            row_height,
            col_width,
            colors,
        }
    }

    /// Draw a single triangle by drawing two sides and filling them with a random color
    /// which creates the third side of the triangle along the way.
    /// Note: Even though the turtle is always turning right in this method, this movement
    /// can also result in a left turn (depending on the angle chosen).
    fn draw_triangle(&mut self, angle: f64) {
        // This call to unwrap() is always safe because the color slice will always be non-empty.
        let color = choose(&self.colors).cloned().unwrap();
        self.turtle.set_fill_color(color);
        self.turtle.begin_fill();

        self.turtle.right(angle);
        self.turtle.forward(self.col_width);

        self.turtle.right(angle);
        self.turtle.forward(self.row_height);

        self.turtle.end_fill();
    }

    /// Draw a single row of triangles in two runs:
    /// 1. Create triangles from left to right.
    /// 2. Turn around and fill in triangles from right to left.
    fn draw_row(&mut self) {
        // Create an endless loop over the angles of 90 and 270 degrees
        // (corresponds to moving right and left, respectively).
        // Need `&mut` so the value does not move when we call `take()` twice
        let angles = &mut [90.0, 270.0].iter().cycle();

        // Create triangles from left to right.
        for angle in angles.take(self.col_count as usize) {
            self.draw_triangle(*angle);
        }

        // Skip one angle so that we have the correct angle when turning around.
        angles.next();

        // Fill in triangles from right to left to complete the row.
        for angle in angles.take(self.col_count as usize) {
            self.draw_triangle(*angle);
        }

        // Reset position to prepare for the next row.
        self.turtle.right(180.0);
        self.turtle.forward(self.row_height);
        self.turtle.left(180.0);
    }

    /// Create a drawing consisting of rows of triangles in different colors.
    fn draw(&mut self) {
        for _ in 0..self.row_count {
            self.draw_row();
        }
        self.turtle.hide();
    }
}
