use std::cell::RefCell;
use std::rc::Rc;

use turtle_window::TurtleWindow;
use {Color, Point};

/// Represents the drawing that the turtle is creating
pub struct Drawing {
    window: Rc<RefCell<TurtleWindow>>,
}

impl Drawing {
    pub(crate) fn with_window(window: Rc<RefCell<TurtleWindow>>) -> Self {
        Self {
            window,
        }
    }

    /// Returns the color of the background.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().set_background_color("purple");
    /// assert_eq!(turtle.drawing().background_color(), "purple".into());
    /// # }
    /// ```
    ///
    /// See the [`color` module](color/index.html) for more information about colors.
    pub fn background_color(&self) -> Color {
        self.window.borrow().fetch_drawing().background
    }

    /// Sets the color of the background to the given color.
    ///
    /// Any type that can be converted into a color can be passed into this function.
    /// See the [`color` module](color/index.html) for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.drawing_mut().set_background_color("orange");
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle background](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/orange_background.png)
    pub fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.background = color.into());
    }

    /// Returns the center of the drawing
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// let mut turtle = Turtle::new();
    /// assert_eq!(turtle.drawing().center(), [0.0, 0.0]);
    /// turtle.drawing_mut().set_center([4.0, 3.0]);
    /// assert_eq!(turtle.drawing().center(), [4.0, 3.0]);
    /// # }
    /// ```
    pub fn center(&self) -> Point {
        self.window.borrow().fetch_drawing().center
    }

    /// Sets the center of the drawing to the given point
    ///
    /// The center represents the offset from the center of the window at which to draw the
    /// drawing. The default center is (0, 0) which means that the drawing is centered at the
    /// center of the window.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    ///
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///
    ///     for _ in 0..360 {
    ///         // Move forward three steps
    ///         turtle.forward(3.0);
    ///         // Rotate to the right (clockwise) by 1 degree
    ///         turtle.right(1.0);
    ///     }
    ///
    ///     turtle.wait_for_click();
    ///     turtle.drawing_mut().set_center([50.0, 100.0]);
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle center middle](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/circle.png)
    ///
    /// Once you click on the window:
    ///
    /// ![turtle center offset](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/circle_offset_center.png)
    pub fn set_center(&mut self, center: Point) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.center = center);
    }

    /// Resets the center of the drawing back to its initial value
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// let mut turtle = Turtle::new();
    /// let default_center = turtle.drawing().center();
    /// turtle.drawing_mut().set_center([400.0, -30.0]);
    /// assert_ne!(turtle.drawing().center(), default_center);
    /// turtle.drawing_mut().reset_center();
    /// assert_eq!(turtle.drawing().center(), default_center);
    /// # }
    /// ```
    pub fn reset_center(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.center = [0.0, 0.0]);
    }
}
