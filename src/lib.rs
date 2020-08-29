//! Use the API Documentation below to learn about the various things you can do with this crate.
//!
//! **If this is your first time using this crate, read the Guide on [turtle.rs](http://turtle.rs)
//! to learn how to start.**
//!
//! * The [`Turtle` struct](struct.Turtle.html) - lists of all the various drawing commands that the
//!   turtle supports
//! * The [`Drawing` struct](struct.Drawing.html) - allows you to manipulate the title, size,
//!   background and more of the drawing that you are creating
//! * The [`Color` struct](struct.Color.html) - describes the different ways to create colors and
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
#![cfg_attr(not(feature = "unstable"), doc = "
**The turtle events API is unstable.** Rebuild the documentation with `--features \"unstable\"` to
show the documentation for the types and modules that are part of that. Some links may appear
broken unless you do so. See [Unstable features](#unstable-features) below for more information.
")]
//! The [`Event` enum](event/enum.Event.html) documentation provides information about how you can
//! create an event loop. This allows you to draw things in response to certain events like the
//! mouse moving, keys being pressed, and more.
//!
//! The `Turtle` struct contains a few convenience methods so you can do some common event-related
//! things without creating the entire event loop. For example, use
//! [`wait_for_click()`](struct.Turtle.html#method.wait_for_click) to wait for the user to click
//! anywhere on the screen before proceeding.
//!
//! # Adding turtle to your project
//!
//! To add the turtle crate to your project, add the following to your `Cargo.toml` file with `...`
//! replaced by the version of the turtle crate you want to use:
//!
//! ```toml
//! [dependencies]
//! turtle = "..."
//!
//! # Compile turtle and other dependencies with optimizations
//! [profile.dev.package."*"]
//! opt-level = 3
//! ```
//!
//! This is the recommended way to use the turtle crate because it provides maximum performance
//! with minimum impact on build time. The initial build will take a bit longer since dependencies
//! are being optimized, but each build of your program after that should run relatively quickly.
//!
//! For maximum overall performance, run your build with the `--release` flag.
//!
//! # Unstable features
//!
//! Some parts of this crate are unstable and may be subject to change in the future. If you would
//! like to use unstable functionality, enable the "unstable" feature in your `Cargo.toml` file as
//! shown below:
//!
//! ```toml
//! [dependencies]
//! # Explicitly opt-in to unstable features that may change in the future
//! turtle = { version = "...", features = ["unstable"] }
//! ```
//!
//! If you are developing this crate with a local version of the turtle repository, see the
//! [CONTRIBUTING.md] file for the specific commands you should run to generate the documentation.
//!
//! [CONTRIBUTING.md]: https://github.com/sunjay/turtle/blob/master/CONTRIBUTING.md

// This warning usually signals an error and so it should be treated as such.
#![deny(unused_must_use)]

#![doc(test(attr(deny(warnings), allow(unused_variables))))]

#![doc(html_logo_url = "https://raw.githubusercontent.com/sunjay/turtle/master/docs/assets/images/turtle-logo-512.png")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(test, not(feature = "test")))]
compile_error!("Make sure you run tests with `cargo test --features \"test unstable\"`");

/// Used to add helper method for doctests to generate pngs
#[cfg(docs_images)]
pub trait SavePng {
    fn save_png(&self, path: &str) -> Result<(), String>;
}

mod radians;
mod point;
mod speed;
mod color;
pub mod rand;

mod ipc_protocol;
mod renderer_server;
mod renderer_client;
mod async_drawing;
mod async_turtle;
mod sync_runtime;
mod debug;
mod drawing;
mod turtle;

pub use crate::color::Color;
pub use crate::color::colors;
pub use crate::async_drawing::Size;
pub use crate::drawing::Drawing;
pub use crate::point::Point;
pub use crate::speed::Speed;
pub use crate::async_turtle::{Angle, Distance};
pub use crate::turtle::Turtle;
pub use crate::renderer_server::{ExportError, start};

cfg_if::cfg_if! {
    if #[cfg(feature = "unstable")] {
        #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
        pub mod event;
        #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
        pub use crate::event::Event;

    } else {
        mod event;
        use crate::event::Event;
    }
}
