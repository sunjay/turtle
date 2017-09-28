extern crate piston_window;
extern crate interpolation;

mod turtle_window;

mod turtle;
mod speed;
mod color;
mod radians;
mod animation;
mod extensions;
mod drawing_thread;

pub use turtle::{Turtle, Point, Distance, Angle};
pub use speed::{Speed};
pub use color::{Color};
