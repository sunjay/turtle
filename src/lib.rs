extern crate piston_window;
extern crate interpolation;
extern crate fps_clock;
extern crate rand;

// Re-exported for the convenience of library users
pub use rand::*;

mod turtle_window;

mod turtle;
mod speed;
mod radians;
mod animation;
mod extensions;
mod renderer;
mod state;
mod event;

pub mod color;

pub use turtle::{Turtle, Point, Distance, Angle};
pub use speed::{Speed};
pub use color::{Color};
pub use event::*;
