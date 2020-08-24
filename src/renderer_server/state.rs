use std::f64::consts::PI;

use serde::{Serialize, Deserialize};

use crate::{
    Color,
    Point,
    Speed,
    debug,
    radians::Radians,
    colors::{WHITE, BLACK},
    async_turtle::AngleUnit,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingState {
    pub title: String,
    pub background: Color,
    pub center: Point,
    pub width: u32,
    pub height: u32,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
}

impl DrawingState {
    pub const DEFAULT_TITLE: &'static str = "Turtle";
    pub const DEFAULT_BACKGROUND: Color = WHITE;
    pub const DEFAULT_CENTER: Point = Point::origin();
    pub const DEFAULT_WIDTH: u32 = 800;
    pub const DEFAULT_HEIGHT: u32 = 600;
    pub const DEFAULT_IS_MAXIMIZED: bool = false;
    pub const DEFAULT_IS_FULLSCREEN: bool = false;
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            title: Self::DEFAULT_TITLE.to_owned(),
            background: Self::DEFAULT_BACKGROUND,
            center: Self::DEFAULT_CENTER,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            is_maximized: Self::DEFAULT_IS_MAXIMIZED,
            is_fullscreen: Self::DEFAULT_IS_FULLSCREEN,
        }
    }
}

impl DrawingState {
    pub(crate) fn to_debug(&self) -> debug::Drawing {
        let &Self {
            ref title,
            background,
            center,
            width,
            height,
            is_maximized,
            is_fullscreen,
        } = self;

        let title = title.clone();

        debug::Drawing {
            title,
            background,
            center,
            width,
            height,
            is_maximized,
            is_fullscreen,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurtleState {
    pub pen: Pen,
    pub fill_color: Color,
    pub position: Point,
    pub heading: Radians,
    pub speed: Speed,
    pub is_visible: bool,
}

impl TurtleState {
    pub const DEFAULT_FILL_COLOR: Color = BLACK;
    pub const DEFAULT_POSITION: Point = Point::origin();
    pub const DEFAULT_HEADING: Radians = Radians::from_radians_value(PI / 2.0);
    pub const DEFAULT_IS_VISIBLE: bool = true;
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            pen: Pen::default(),
            fill_color: Self::DEFAULT_FILL_COLOR,
            position: Self::DEFAULT_POSITION,
            heading: Self::DEFAULT_HEADING,
            speed: Speed::default(),
            is_visible: Self::DEFAULT_IS_VISIBLE,
        }
    }
}

impl TurtleState {
    pub(crate) fn to_debug(&self, angle_unit: AngleUnit) -> debug::Turtle {
        let &Self {
            position,
            heading,
            speed,
            ref pen,
            fill_color,
            is_visible,
        } = self;

        let heading = match angle_unit {
            AngleUnit::Degrees => debug::DebugAngle::Degrees(heading.to_degrees()),
            AngleUnit::Radians => debug::DebugAngle::Radians(heading.to_radians()),
        };
        let pen = pen.to_debug();

        debug::Turtle {
            position,
            heading,
            speed,
            pen,
            fill_color,
            is_visible,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pen {
    pub is_enabled: bool,
    pub thickness: f64,
    pub color: Color,
}

impl Pen {
    pub const DEFAULT_IS_ENABLED: bool = true;
    pub const DEFAULT_THICKNESS: f64 = 1.0;
    pub const DEFAULT_COLOR: Color = BLACK;
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            is_enabled: Self::DEFAULT_IS_ENABLED,
            thickness: Self::DEFAULT_THICKNESS,
            color: Self::DEFAULT_COLOR,
        }
    }
}

impl Pen {
    pub(crate) fn to_debug(&self) -> debug::Pen {
        let &Self {
            is_enabled,
            thickness,
            color,
        } = self;

        debug::Pen {
            is_enabled,
            thickness,
            color,
        }
    }
}
