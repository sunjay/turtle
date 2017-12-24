use std::cell::RefCell;
use std::rc::Rc;

use turtle_window::TurtleWindow;
use runtime::Runtime;
use {Color};

/// Represents the drawing that the turtle is creating
pub struct Drawing<R: Runtime> {
    window: Rc<RefCell<TurtleWindow<R>>>,
}

impl<R: Runtime> Drawing<R> {
    pub(crate) fn with_window(window: Rc<RefCell<TurtleWindow<R>>>) -> Self {
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
    /// # turtle::start_desktop(|mut turtle| {
    /// turtle.drawing_mut().set_background_color("purple");
    /// assert_eq!(turtle.drawing().background_color(), "purple".into());
    /// # });}
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
    /// # extern crate turtle;
    /// # use turtle::Turtle;
    ///
    /// # fn main() {
    /// # turtle::start_desktop(|mut turtle| {
    /// turtle.drawing_mut().set_background_color("orange");
    /// # });}
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle background](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/orange_background.png)
    pub fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.background = color.into());
    }
}
