//! Use the API Documentation below to learn about the various things you can do with this crate.
//!
//! **If this is your first time using Rust or using this crate, read the Guide on
//! [turtle.rs](http://turtle.rs) to learn how to start.**
//!
//! * The [`Turtle` struct](struct.Turtle.html) - lists of all the various drawing commands that the
//!   turtle supports
//! * The [`Drawing` struct](struct.Drawing.html) - allows you to manipulate the title, size,
//!   background and more of the drawing that you are creating
//! * The [`color` module](color/index.html) - describes the different ways to create colors and
//!   includes a list of the hundreds of predefined color names that you can use to easily set the
//!   pen, fill, and background color of your drawings
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
//! # Unstable features
//! Some parts of this library are unstable, such as maximizing and unmaximizing the window.
//! You can explicitly opt-in to using those features with the `unstable` feature like so:
//!
//! ```bash
//! $ cargo build --features "unstable"
//! ```
//!
//! If you want to use this from inside your own crate, you will need to add this to your Cargo.toml
//! ```toml
//! [dependencies]
//! turtle = { version = "...", features = ["unstable"] }
//! ```

// This warning usually signals an error and so it should be treated as such.
#![deny(unused_must_use)]

#![doc(html_logo_url = "https://raw.githubusercontent.com/sunjay/turtle/master/docs/assets/images/turtle-logo-512.png")]
#![cfg_attr(target_arch = "wasm32", crate_type = "cdylib")]

#[cfg(all(test, not(feature = "test")))]
compile_error!("Make sure you run tests with `cargo test --features test`");

mod ipc_protocol;
mod renderer_server;
mod renderer_client;
mod async_drawing;
mod async_turtle;
mod sync_runtime;
mod turtle;

mod turtle_window;

mod animation;
#[cfg(not(target_arch = "wasm32"))]
mod app;
mod drawing;
mod extensions;
#[cfg(not(target_arch = "wasm32"))]
mod messenger;
mod point;
mod query;
mod radians;
#[cfg(not(target_arch = "wasm32"))]
mod renderer2;
mod renderer_process;
#[cfg(not(target_arch = "wasm32"))]
mod server;
mod speed;
mod state2;
mod timer;

pub mod color;
#[cfg(not(target_arch = "wasm32"))]
pub mod event;
#[cfg(target_arch = "wasm32")]
mod event {
    use serde::{Serialize, Deserialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Event {}
}
pub mod rand;

pub use crate::color::Color;
pub use crate::async_drawing::Size;
pub use crate::drawing::Drawing;
pub use crate::event::Event;
pub use crate::point::Point;
#[cfg(target_arch = "wasm32")]
pub use renderer_process::{alloc, dealloc, dealloc_str};
pub use crate::speed::Speed;
pub use crate::async_turtle::{Angle, Distance};
pub use crate::turtle::Turtle;
use crate::renderer_server::ExportError;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::renderer_server::start;
#[cfg(target_arch = "wasm32")]
pub fn start() {}
