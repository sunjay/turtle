use std::cell::RefCell;
use std::rc::Rc;

use turtle_window::TurtleWindow;
use state::DrawingState;
use {Color, Point};

/// Represents a size
///
/// A `Size` can be converted from either a tuple or array. These forms are often more ergonomic
/// than using the `Size` struct on its own. The [`set_size()`](struct.Drawing.html#method.set_size)
/// method accepts either form (without needing to use `into()`). See that method's documentation
/// for more information.
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Size;
/// # fn main() {
/// assert_eq!(Size {width: 640, height: 480}, (640, 480).into());
/// assert_eq!(Size {width: 640, height: 480}, [640, 480].into());
/// # }
/// ```
///
/// You can access the `width` and `height` fields directly on any `Size` struct.
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Size;
/// # fn main() {
/// let size: Size = (800, 600).into();
/// assert_eq!(size.width, 800);
/// assert_eq!(size.height, 600);
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    /// The width in pixels
    pub width: u32,
    /// The height in pixels
    pub height: u32,
}

impl From<(u32, u32)> for Size {
    fn from(size: (u32, u32)) -> Self {
        Self {
            width: size.0,
            height: size.1,
        }
    }
}

impl From<[u32; 2]> for Size {
    fn from(size: [u32; 2]) -> Self {
        Self {
            width: size[0],
            height: size[1],
        }
    }
}

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

    /// Returns the title of the drawing
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().set_title("Hello, world!");
    /// assert_eq!(&*turtle.drawing().title(), "Hello, world!");
    /// # }
    /// ```
    pub fn title(&self) -> String {
        self.window.borrow().fetch_drawing().title
    }

    /// Sets the title of the drawing to the given text
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// extern crate turtle;
    /// use turtle::Turtle;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.drawing_mut().set_title("My Fancy Title! - Yay!");
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle set title](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/changed_title.png)
    pub fn set_title(&mut self, title: &str) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.title = title.to_owned());
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
    /// The center represents the offset from the center of the viewport at which to draw the
    /// drawing. The default center is (0, 0) which means that the drawing is centered at the
    /// middle of the viewport.
    ///
    /// Use this method to move the canvas that the turtle is drawing on.
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
        let default = DrawingState::default();
        self.set_center(default.center);
    }

    /// Returns the size of the drawing
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// # assert_eq!(turtle.drawing().size(), Size {width: 800, height: 600});
    /// // 1080p is 1920x1080
    /// turtle.drawing_mut().set_size((1920, 1080));
    /// let size = turtle.drawing().size();
    /// assert_eq!(size.width, 1920);
    /// assert_eq!(size.height, 1080);
    /// # }
    /// ```
    pub fn size(&self) -> Size {
        let drawing = self.window.borrow().fetch_drawing();
        Size {
            width: drawing.width,
            height: drawing.height,
        }
    }

    /// Sets the size of the drawing to the given width and height.
    ///
    /// You can specify the size as any value that can be converted into a
    /// [`Size` struct](struct.Size.html). That means that any of the following would work:
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::Turtle;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// // These are all equivalent to each other
    /// turtle.drawing_mut().set_size((640, 480));
    /// # assert_eq!(turtle.drawing().size(), Size {width: 640, height: 480});
    /// turtle.drawing_mut().set_size([640, 480]);
    /// # assert_eq!(turtle.drawing().size(), Size {width: 640, height: 480});
    ///
    /// // It recommended that you use the options above instead of the following
    /// use turtle::Size; // Size must be imported
    /// turtle.drawing_mut().set_size(Size {width: 640, height: 480});
    /// # assert_eq!(turtle.drawing().size(), Size {width: 640, height: 480});
    /// # }
    /// ```
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
    ///     turtle.drawing_mut().set_size((300, 300));
    /// }
    /// ```
    ///
    /// This will produce the following:
    ///
    /// ![turtle default size](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/circle.png)
    ///
    /// Once you click on the window:
    ///
    /// ![turtle small drawing](https://github.com/sunjay/turtle/raw/gh-pages/assets/images/docs/small_drawing.png)
    ///
    /// Notice that the center of the drawing stays the same. To control that, see
    /// [`set_center()`](struct.Drawing.html#method.set_center).
    pub fn set_size<S: Into<Size>>(&mut self, size: S) {
        let size = size.into();
        self.window.borrow_mut().with_drawing_mut(|drawing| {
            drawing.width = size.width;
            drawing.height = size.height;
        });
    }

    /// Resets the size of the drawing back to its initial value
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// let mut turtle = Turtle::new();
    /// let default_size = turtle.drawing().size();
    ///
    /// turtle.drawing_mut().set_size([920, 680]);
    /// assert_ne!(turtle.drawing().size(), default_size);
    ///
    /// turtle.drawing_mut().reset_size();
    /// assert_eq!(turtle.drawing().size(), default_size);
    /// # }
    /// ```
    pub fn reset_size(&mut self) {
        let default = DrawingState::default();
        self.set_size((default.width, default.height));
    }

    /// Returns true if the drawing is currently maximized.
    ///
    /// Note: Even if you set the drawing to the width and height of the current display, it won't
    /// be maximized unless [`maximize()`](struct.Drawing.html#method.maximize) is called.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// let mut turtle = Turtle::new();
    /// assert_eq!(turtle.drawing().is_maximized(), false);
    ///
    /// turtle.drawing_mut().maximize();
    /// assert_eq!(turtle.drawing().is_maximized(), true);
    ///
    /// turtle.drawing_mut().unmaximize();
    /// assert_eq!(turtle.drawing().is_maximized(), false);
    ///
    /// // Calling the same method again doesn't change the result
    /// turtle.drawing_mut().unmaximize();
    /// assert_eq!(turtle.drawing().is_maximized(), false);
    /// # }
    /// ```
    ///
    /// # Unstable
    ///
    /// This method is currently unstable and unreliable. Unfortunately, we cannot currently detect
    /// when the window is maximized using the maximize button on the window. This is a limitation
    /// that will eventually be lifted. Until then, consider the return value of this method
    /// unreliable.
    pub fn is_maximized(&self) -> bool {
        self.window.borrow().fetch_drawing().maximized
    }

    /// Maximizes the size of the drawing so that it takes up the entire display.
    ///
    /// If the drawing is already maximized, this method does nothing.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().maximize();
    /// assert_eq!(turtle.drawing().is_maximized(), true);
    /// // Calling this method again does nothing
    /// turtle.drawing_mut().maximize();
    /// assert_eq!(turtle.drawing().is_maximized(), true);
    /// # }
    /// ```
    ///
    /// # Unstable
    ///
    /// This method is currently unstable and unreliable. Unfortunately, we cannot currently detect
    /// when the window is maximized using the maximize button on the window. This is a limitation
    /// that will eventually be lifted. Until then, consider the behaviour of this method unreliable.
    ///
    /// It is usually okay to use this method right when the turtle is created, but don't rely on
    /// it after that because by then the user may have pressed the maximize button on the window.
    pub fn maximize(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.maximized = true);
    }

    /// Returns the size of the drawing to its value before it was maximized
    ///
    /// If the drawing is already unmaximized, this method does nothing.
    ///
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().maximize();
    /// assert_eq!(turtle.drawing().is_maximized(), true);
    /// turtle.drawing_mut().unmaximize();
    /// assert_eq!(turtle.drawing().is_maximized(), false);
    /// // Calling this again does nothing because the drawing is already unmaximized
    /// turtle.drawing_mut().unmaximize();
    /// assert_eq!(turtle.drawing().is_maximized(), false);
    /// # }
    /// ```
    ///
    /// # Unstable
    ///
    /// This method is currently unstable and unreliable. Unfortunately, we cannot currently detect
    /// when the window is maximized using the maximize button on the window. This is a limitation
    /// that will eventually be lifted. Until then, consider the behaviour of this method unreliable.
    pub fn unmaximize(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.maximized = false);
    }

    /// Returns true if the drawing is currently full screen.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// let mut turtle = Turtle::new();
    /// assert_eq!(turtle.drawing().is_fullscreen(), false);
    ///
    /// turtle.drawing_mut().enter_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), true);
    ///
    /// turtle.drawing_mut().exit_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), false);
    ///
    /// // Calling the same method again doesn't change the result
    /// turtle.drawing_mut().exit_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), false);
    /// # }
    /// ```
    pub fn is_fullscreen(&self) -> bool {
        self.window.borrow().fetch_drawing().fullscreen
    }

    /// Makes the drawing take up the entire screen hiding everything else that would have been
    /// shown on screen otherwise.
    ///
    /// If the drawing is already fullscreen, this method does nothing.
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().enter_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), true);
    /// // Calling this method again does nothing
    /// turtle.drawing_mut().enter_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), true);
    /// # }
    /// ```
    pub fn enter_fullscreen(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.fullscreen = true);
    }

    /// Returns the size of the drawing to its value before it became fullscreen.
    ///
    /// If the drawing is already not fullscreen, this method does nothing.
    ///
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().enter_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), true);
    /// turtle.drawing_mut().exit_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), false);
    /// // Calling this again does nothing because the drawing is already not fullscreen
    /// turtle.drawing_mut().exit_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), false);
    /// # }
    /// ```
    pub fn exit_fullscreen(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.fullscreen = false);
    }
}
