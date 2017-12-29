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

// wasm needs an alloc and dealloc
#![cfg_attr(target_arch = "wasm32", feature(allocator_api))]

#[cfg(all(test, not(feature = "test")))]
compile_error!("Make sure you run tests with `cargo test --features test`");

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
mod drawing;
mod speed;
mod radians;
mod animation;
mod extensions;
mod state;
mod query;
mod renderer;
mod runtime;
mod clock;
#[cfg(feature = "desktop")]
mod desktop;
#[cfg(feature = "canvas")]
mod canvas;

pub mod color;
pub mod event;
pub mod rand;

pub use turtle::{Turtle, Point, Distance, Angle};
pub use drawing::{Drawing, Size};
pub use speed::{Speed};
pub use color::{Color};
pub use event::{Event};

#[cfg(feature = "desktop")]
type DefaultRuntime = ::desktop::DesktopRuntime;
#[cfg(feature = "canvas")]
type DefaultRuntime = ::canvas::CanvasRuntime;

#[cfg(feature = "canvas")]
pub use canvas::{alloc, dealloc};

#[cfg(any(feature = "desktop", feature = "test"))]
pub fn start_desktop<F>(f: F) where F: FnOnce(Turtle) {
    let turtle = Turtle::new(desktop::DesktopRuntime::new());

    f(turtle);
}

#[cfg(feature = "canvas")]
pub fn start_web<F>(pointer: *mut u8, width: usize, height: usize, f: F) where F: FnOnce(Turtle) {
    let turtle =
        Turtle::new(canvas::CanvasRuntime::new(width, height, pointer));

    f(turtle);
}

#[macro_export]
macro_rules! run_turtle {
    ($f:expr) => {
        #[cfg(feature = "desktop")]
        fn main() {
            turtle::start_desktop($f);
        }

        #[cfg(feature = "canvas")]
        fn main() {
            println!("unused in wasm");
        }

        #[cfg(feature = "canvas")]
        #[allow(dead_code)]
        #[no_mangle]
        pub extern "C" fn web_turtle_start(pointer: *mut u8, width: usize, height: usize) {
            turtle::start_web(pointer, width, height, $f);
        }
    }
}
