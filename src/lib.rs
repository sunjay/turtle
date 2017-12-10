//! The API Documentation below will help you learn about the various things you can do with this
//! crate.
//!
//! **If this is your first time using Rust or using this crate, read the Guide on
//! [turtle.rs](http://turtle.rs) to learn how to start.**
//!
//! The documentation for the [`Turtle` struct](struct.Turtle.html) contains a listing of all of the
//! various drawing commands that you can use on the Turtles that you create. The [`color`
//! module](color/index.html) and its submodules contain hundreds of predefined color names that you
//! can use to set the pen, background and fill color of a turtle. That module also explains how to
//! use any arbitrary color, including ones that may not be defined in this crate.
//!
//! Note: Call [`turtle::start()`](fn.start.html) if you do not create a turtle with
//! [`Turtle::new()`](struct.Turtle.html#method.new) right at the beginning of your program. Most
//! programs will never need to call this function as it is called for you in
//! [`Turtle::new()`](struct.Turtle.html#method.new).
//!
//! # Random Values
//!
//! See the [`rand` module](rand/index.html) for information about generating random colors, speeds,
//! angles, and more which can be used in your programs to produce some interesting results!
//!
//! # Event Handling
//!
//! The [`Event` enum](event/enum.Event.html) documentation provides information about how you can
//! create an event loop. This allows you to draw things in response to certain events like the
//! mouse moving, keys being pressed, and more.
//!
//! The `Turtle` struct contains a few convenience methods so you can do some common event-related
//! things without creating the entire event loop. For example, use
//! [`wait_for_click()`](struct.Turtle.html#method.wait_for_click) to wait for the user to click
//! anywhere on the screen before proceeding.

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "desktop")]
extern crate piston_window;
extern crate graphics;
extern crate input;
extern crate interpolation;

extern crate rand as rand_crate;

mod turtle_window;

mod app;
mod turtle;
mod speed;
mod radians;
mod animation;
mod extensions;
mod state;
mod query;
mod messenger;
mod renderer;
mod render_strategy;
#[cfg(feature = "desktop")]
mod desktop;
#[cfg(feature = "canvas")]
mod canvas;

pub mod color;
pub mod event;
pub mod rand;

pub use turtle::{Turtle, Point, Distance, Angle};
pub use speed::{Speed};
pub use color::{Color};
pub use event::{Event};
pub use rand::{random, random_range};

#[cfg(feature = "desktop")]
type DefaultRenderStrategy = ::desktop::DesktopRenderStrategy;
#[cfg(feature = "canvas")]
type DefaultRenderStrategy = ::canvas::CanvasRenderStrategy;


/// Set up turtle rendering.
///
/// If you do not create a turtle immediately at the beginning of `main()` with [`Turtle::new()`],
/// you must **call this function at the start of `main()` to avoid any problems**.
///
/// Since the majority of code created using this crate does little or no work before calling
/// `Turtle::new()`, this usually isn't a problem. Programs that parse command line arguments, read
/// input, or check environment variables may **fail** to start if this function is not called
/// right at the beginning of the program. Programs that perform any expensive computations may
/// experience delayed start up problems unless they call this function first.
///
/// The [`Turtle::new()`] method will call this function for you so that you don't need to worry
/// about this unless you are doing something before that.
///
/// # Example
/// ```rust,no_run
/// # #![allow(unused_variables, unused_mut)]
/// extern crate turtle;
/// use turtle::Turtle;
///
/// fn main() {
///     // Initializes the turtle renderer first so that there is less delay when a Turtle
///     // is created and so that there are no conflicts with command line arguments or
///     // environment variables.
///     // Not required if Turtle::new() is already at the top of main.
///     turtle::start();
///
///     // Do all kinds of expensive work here...
///     // Feel free to check environment variables, command line arguments, etc.
///
///     // Create the turtle when you are ready
///     // Turtle::new() will also call start(), but calling it twice doesn't matter
///     let mut turtle = Turtle::new();
///     // Do things with the turtle...
/// }
/// ```
///
/// [`Turtle::new()`]: struct.Turtle.html#method.new
pub fn start() {
    use render_strategy::RenderStrategy;;
    DefaultRenderStrategy::initialize()
}
