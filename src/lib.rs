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

#[cfg(all(test, not(feature = "test")))]
compile_error!("Make sure you run tests with `cargo test --features test`");

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate piston_window;
extern crate interpolation;
extern crate rand as rand_crate;

mod turtle_window;

mod app;
mod turtle;
mod speed;
mod radians;
mod animation;
mod extensions;
mod renderer;
mod state;
mod query;
mod server;
mod renderer_process;
mod messenger;

pub mod color;
pub mod event;
pub mod rand;

pub use server::start;
pub use turtle::{Turtle, Point, Distance, Angle};
pub use speed::{Speed};
pub use color::{Color};
pub use event::Event;
pub use rand::{random, random_range};
