use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::path::Path;

use crate::state::DrawingState;
use crate::turtle_window::TurtleWindow;
use crate::{Color, Event, Point};

/// Represents a size
///
/// A `Size` can be converted from either a tuple or array. These forms are often more ergonomic
/// than using the `Size` struct on its own. The [`set_size()`](struct.Drawing.html#method.set_size)
/// method accepts either form (without needing to use `into()`). See that method's documentation
/// for more information.
///
/// ```rust
/// # use turtle::Size;
/// assert_eq!(Size {width: 640, height: 480}, (640, 480).into());
/// assert_eq!(Size {width: 640, height: 480}, [640, 480].into());
/// ```
///
/// You can access the `width` and `height` fields directly on any `Size` struct.
///
/// ```rust
/// # use turtle::Size;
/// let size: Size = (800, 600).into();
/// assert_eq!(size.width, 800);
/// assert_eq!(size.height, 600);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
///
/// This struct is usually accessed using the [`drawing()`](struct.Turtle.html#method.drawing)
/// and [`drawing_mut()`](struct.Turtle.html#method.drawing_mut) methods on the
/// [`Turtle` struct](struct.Turtle.html).
pub struct Drawing {
    window: Rc<RefCell<TurtleWindow>>,
}

impl Drawing {
    pub(crate) fn with_window(window: Rc<RefCell<TurtleWindow>>) -> Self {
        Self { window }
    }

    /// Returns the title of the drawing
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().set_title("Hello, world!");
    /// assert_eq!(&*turtle.drawing().title(), "Hello, world!");
    /// ```
    pub fn title(&self) -> String {
        self.window.borrow().fetch_drawing().title
    }

    /// Sets the title of the drawing to the given text
    ///
    /// # Example
    ///
    /// ```rust,no_run
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
    /// ![turtle set title](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/changed_title.png)
    pub fn set_title(&mut self, title: &str) {
        self.window
            .borrow_mut()
            .with_drawing_mut(|drawing| drawing.title = title.to_owned());
    }

    /// Returns the color of the background.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().set_background_color("purple");
    /// assert_eq!(turtle.drawing().background_color(), "purple".into());
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
    /// ![turtle background](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/orange_background.png)
    pub fn set_background_color<C: Into<Color> + Copy + Debug>(&mut self, color: C) {
        let bg_color = color.into();
        assert!(
            bg_color.is_valid(),
            "Invalid color: {:?}. See the color module documentation for more information.",
            color
        );
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.background = bg_color);
    }

    /// Returns the center of the drawing
    ///
    /// ```rust
    /// # use turtle::*;
    /// let mut turtle = Turtle::new();
    /// assert_eq!(turtle.drawing().center(), Point {x: 0.0, y: 0.0});
    /// turtle.drawing_mut().set_center([4.0, 3.0]);
    /// assert_eq!(turtle.drawing().center(), Point {x: 4.0, y: 3.0});
    /// ```
    pub fn center(&self) -> Point {
        self.window.borrow().fetch_drawing().center
    }

    /// Sets the center of the drawing to the given point. See the [`Point` struct](struct.Point.html)
    /// documentation for more information.
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
    /// ![turtle center middle](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/circle.png)
    ///
    /// Once you click on the window:
    ///
    /// ![turtle center offset](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/circle_offset_center.png)
    pub fn set_center<P: Into<Point>>(&mut self, center: P) {
        let center = center.into();
        if !center.is_finite() {
            return;
        }
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.center = center);
    }

    /// Resets the center of the drawing back to its initial value
    ///
    /// ```rust
    /// # use turtle::*;
    /// let mut turtle = Turtle::new();
    /// let default_center = turtle.drawing().center();
    /// turtle.drawing_mut().set_center([400.0, -30.0]);
    /// assert_ne!(turtle.drawing().center(), default_center);
    /// turtle.drawing_mut().reset_center();
    /// assert_eq!(turtle.drawing().center(), default_center);
    /// ```
    pub fn reset_center(&mut self) {
        let default = DrawingState::default();
        self.set_center(default.center);
    }

    /// Returns the size of the drawing
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// # assert_eq!(turtle.drawing().size(), Size {width: 800, height: 600});
    /// // 1080p is 1920x1080
    /// turtle.drawing_mut().set_size((1920, 1080));
    /// let size = turtle.drawing().size();
    /// assert_eq!(size.width, 1920);
    /// assert_eq!(size.height, 1080);
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
    /// # use turtle::Turtle;
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
    /// ```
    ///
    /// # Example
    ///
    /// ```rust,no_run
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
    /// ![turtle default size](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/circle.png)
    ///
    /// Once you click on the window:
    ///
    /// ![turtle small drawing](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/small_drawing.png)
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
    /// # use turtle::*;
    /// let mut turtle = Turtle::new();
    /// let default_size = turtle.drawing().size();
    ///
    /// turtle.drawing_mut().set_size([920, 680]);
    /// assert_ne!(turtle.drawing().size(), default_size);
    ///
    /// turtle.drawing_mut().reset_size();
    /// assert_eq!(turtle.drawing().size(), default_size);
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
    /// # use turtle::*;
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
    /// ```
    ///
    /// # Unstable
    ///
    /// **This method is currently unstable and unreliable.**
    /// ([GitHub Issue](https://github.com/sunjay/turtle/issues/49))
    ///
    /// Unfortunately, we cannot currently detect when the window is maximized using the maximize
    /// button on the window. This method is reliable until that button is pressed. Since there is
    /// no way to tell when that is, treat the value returned from this method as unreliable and
    /// potentially inaccurate.
    #[cfg(feature = "unstable")]
    pub fn is_maximized(&self) -> bool {
        self.window.borrow().fetch_drawing().maximized
    }

    /// Maximizes the size of the drawing so that it takes up the entire display.
    ///
    /// If the drawing is already maximized, this method does nothing.
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().maximize();
    /// assert_eq!(turtle.drawing().is_maximized(), true);
    /// // Calling this method again does nothing
    /// turtle.drawing_mut().maximize();
    /// assert_eq!(turtle.drawing().is_maximized(), true);
    /// ```
    ///
    /// # Unstable
    ///
    /// **This method is currently unstable and unreliable.**
    /// ([GitHub Issue](https://github.com/sunjay/turtle/issues/49))
    ///
    /// Unfortunately, we cannot currently detect when the window is maximized using the maximize
    /// button on the window. This method is reliable until that button is pressed. Since there is
    /// no way to tell when that is, treat the value returned from this method as unreliable and
    /// potentially inaccurate.
    ///
    /// It is usually okay to use this method right when the turtle is created, but don't rely on
    /// it after that because by then the user may have pressed the maximize button on the window.
    #[cfg(feature = "unstable")]
    pub fn maximize(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.maximized = true);
    }

    /// Returns the size of the drawing to its value before it was maximized
    ///
    /// If the drawing is already unmaximized, this method does nothing.
    ///
    ///
    /// ```rust
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().maximize();
    /// assert_eq!(turtle.drawing().is_maximized(), true);
    /// turtle.drawing_mut().unmaximize();
    /// assert_eq!(turtle.drawing().is_maximized(), false);
    /// // Calling this again does nothing because the drawing is already unmaximized
    /// turtle.drawing_mut().unmaximize();
    /// assert_eq!(turtle.drawing().is_maximized(), false);
    /// ```
    ///
    /// # Unstable
    ///
    /// **This method is currently unstable and unreliable.**
    /// ([GitHub Issue](https://github.com/sunjay/turtle/issues/49))
    ///
    /// Unfortunately, we cannot currently detect when the window is maximized using the maximize
    /// button on the window. This method is reliable until that button is pressed. Since there is
    /// no way to tell when that is, treat the value returned from this method as unreliable and
    /// potentially inaccurate.
    #[cfg(feature = "unstable")]
    pub fn unmaximize(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.maximized = false);
    }

    /// Returns true if the drawing is currently full screen.
    ///
    /// ```rust
    /// # use turtle::*;
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
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().enter_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), true);
    /// // Calling this method again does nothing
    /// turtle.drawing_mut().enter_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), true);
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
    /// # use turtle::*;
    /// # let mut turtle = Turtle::new();
    /// turtle.drawing_mut().enter_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), true);
    /// turtle.drawing_mut().exit_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), false);
    /// // Calling this again does nothing because the drawing is already not fullscreen
    /// turtle.drawing_mut().exit_fullscreen();
    /// assert_eq!(turtle.drawing().is_fullscreen(), false);
    /// ```
    pub fn exit_fullscreen(&mut self) {
        self.window.borrow_mut().with_drawing_mut(|drawing| drawing.fullscreen = false);
    }

    /// Returns the next event (if any). Returns `None` if there are no events to be processed at
    /// the current moment. This **does not** mean that there will never be events later on as the
    /// application continues to run.
    ///
    /// See the [`Event` enum](event/enum.Event.html) for the complete list of events that you can
    /// handle in your applications.
    ///
    /// # Example
    ///
    /// To use this advanced method, you need to create what is known as an "event loop". An "event
    /// loop" is any loop that handles the events generated by the application. The reason that it
    /// is important to create a loop like this is because events in Turtle are "polled". That
    /// means that every time an event happens, it is placed in a queue (a list) until you ask to
    /// look at it. If you do not check for events continuously, there is a chance that the events
    /// you ask for from `poll_event()` will be outdated.
    ///
    /// Even if you do not use every kind of event, you should aim to poll events using this method
    /// until there are none left to poll. If you do not poll events for a significant amount of
    /// time during your application, favor the events that come later as you poll since those will
    /// be the most recent. This can happen if you run many animations between loop iterations.
    ///
    /// See the [`examples/`](https://github.com/sunjay/turtle/raw/master/examples) directory in
    /// the source code of this library for more examples of how to use events.
    ///
    /// The following is an example of a basic event loop. Notice that it uses two loops. One to
    /// move the turtle continuously, and another to handle all the events available at a given
    /// moment. If it suits your purposes, you may also just use a single loop to handle events
    /// and move the turtle from within that loop. This example is of a more complex case where
    /// it really matters that the most recent information is taken into consideration before any
    /// further movements take place.
    ///
    /// ```rust,no_run
    /// use turtle::Turtle;
    /// use turtle::event::Key::{Left, Right};
    /// use turtle::Event::KeyPressed;
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///
    ///     loop {
    ///         turtle.forward(1.0);
    ///
    ///         while let Some(event) = turtle.drawing_mut().poll_event() {
    ///             match event {
    ///                 KeyPressed(key) => match key {
    ///                     Left => {
    ///                         turtle.set_speed(8);
    ///                         for _ in 0..20 {
    ///                             turtle.forward(1.0);
    ///                             turtle.left(4.5);
    ///                         }
    ///                         turtle.set_speed(4);
    ///                     },
    ///                     Right => {
    ///                         turtle.set_speed(8);
    ///                         for _ in 0..20 {
    ///                             turtle.forward(1.0);
    ///                             turtle.right(4.5);
    ///                         }
    ///                         turtle.set_speed(4);
    ///                     },
    ///                     _ => {},
    ///                 },
    ///                 _ => {},
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn poll_event(&mut self) -> Option<Event> {
        self.window.borrow_mut().poll_event()
    }

    /// Saves the current drawings in SVG format at the location specified by `path`.
    ///
    /// ```rust
    /// use turtle::{Turtle, Color};
    ///
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.drawing_mut().set_background_color("pink");
    ///
    ///     for i in 0..36 {
    ///         let base_color: Color = if i % 2 == 0 {
    ///             "red".into()
    ///         } else {
    ///             "white".into()
    ///         };
    ///         turtle.set_fill_color(base_color.with_alpha(1.0 - i as f64 / 54.0));
    ///         turtle.begin_fill();
    ///         square(&mut turtle);
    ///         turtle.end_fill();
    ///         turtle.right(10.0);
    ///     }
    ///    turtle.hide();
    ///    turtle.drawing().save_svg("squares.svg");
    /// }
    ///
    /// fn square(turtle: &mut Turtle) {
    ///     for _ in 0..4 {
    ///         turtle.forward(200.0);
    ///         turtle.right(90.0);
    ///     }
    /// }
    /// ```
    ///
    /// This will produce the following image in the current directory under the name `squares.svg`:
    ///
    /// #TODO: substitute actual link to squares.svg
    /// ![squares](link to docs/assets/images/docs/squares.svg)
    pub fn save_svg<P: AsRef<Path>>(&self, path: P) {
        self.window.borrow().save_svg(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::turtle::*;

    #[test]
    #[should_panic(
        expected = "Invalid color: Color { red: NaN, green: 0.0, blue: 0.0, alpha: 0.0 }. See the color module documentation for more information."
    )]
    fn rejects_invalid_background_color() {
        let mut turtle = Turtle::new();
        turtle.drawing_mut().set_background_color(Color {
            red: ::std::f64::NAN,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        });
    }
}
