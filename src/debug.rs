//! Debug representations of the turtle and drawing state
//!
//! These are not required to exactly match our internal representation of the
//! state. Instead, we should focus on making a printable representation that is
//! as useful for debugging as possible.
//!
//! To avoid confusion, these types are not usually imported by name. Instead,
//! it's best to import the `turtle::debug` module and use the types through
//! that. Example: `debug::Turtle`, `debug::Drawing`, etc.

use std::fmt;

use serde::{Serialize, Deserialize};

use crate::{Color, Point, Speed};

// None of the struct fields are public because we don't want to expose any
// internal details. These types are for printing only!

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drawing {
    pub(crate) title: String,
    pub(crate) background: Color,
    pub(crate) center: Point,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) is_maximized: bool,
    pub(crate) is_fullscreen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turtle {
    pub(crate) position: Point,
    pub(crate) heading: DebugAngle,
    pub(crate) speed: Speed,
    pub(crate) pen: Pen,
    pub(crate) fill_color: Color,
    pub(crate) is_visible: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum DebugAngle {
    /// An angle in degrees
    Degrees(f64),
    /// An angle in radians
    Radians(f64),
}

impl fmt::Debug for DebugAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DebugAngle::*;
        match self {
            Degrees(value) => write!(f, "{}\u{00B0}", value),
            Radians(value) => write!(f, "{} rad", value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pen {
    pub(crate) is_enabled: bool,
    pub(crate) thickness: f64,
    pub(crate) color: Color,
}
