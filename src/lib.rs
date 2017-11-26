//! The API Documentation below will help you learn about the various things you can do with this
//! library.
//!
//! **If this is your first time using Rust or using this library, read the Guide on
//! [turtle.rs](http://turtle.rs) to learn how to start.**
//!
//! The [`Turtle` struct](struct.Turtle.html) documentation contains information about the various
//! things you can do with the `Turtle` instances you create. The [`color` module](color/index.html)
//! and its submodules contain hundreds of constants that you can use as color values.
//! As a convenience, you can use the color name listed with each constant in a string instead of the
//! constant itself. See that module for more information about how to use different colors.
//!
//! # Random Values
//!
//! See the [`rand` module](rand/index.html) for information about generating random values which
//! can be used in your programs.
//!
//! # Event Handling
//!
//! For advanced users, the [`Event` enum](event/enum.Event.html) documentation provides information
//! about how you can create an event loop. This allows you to draw things in response to
//! certain events like the mouse moving, keys being pressed, and more.
//!
//! The `Turtle` struct contains a few convenience methods so you can do some common event-related
//! things without creating the entire event loop. For example, use
//! [`wait_for_click()`](struct.Turtle.html#method.wait_for_click) to wait for the user to click
//! anywhere on the screen before proceeding.

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
mod canvas;
mod speed;
mod radians;
mod animation;
mod extensions;
mod renderer;
mod state;
mod query;
mod types;

pub mod color;
pub mod event;
pub mod rand;

pub use turtle::Turtle;
pub use types::{Point, Distance, Angle};
pub use speed::{Speed};
pub use color::{Color};
pub use event::Event;
pub use rand::{random, random_range};
