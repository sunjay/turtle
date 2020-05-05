use std::fmt::Debug;

use tokio::time;

use crate::radians::{self, Radians};
use crate::ipc_protocol::{ProtocolClient, RotationDirection};
use crate::renderer_server::TurtleId;
use crate::{Turtle, Color, Drawing, Point, Speed};

/// Any distance value (positive or negative)
pub type Distance = f64;

/// An angle value without a unit
///
/// The unit with which this angle will be interpreted depends on whether the Turtle is set to use
/// degrees or radians. See the [`use_degrees()`](struct.Turtle.html#method.use_degrees) or
/// [`use_radians()`](struct.Turtle.html#method.use_radians) methods for more information.
pub type Angle = f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AngleUnit {
    Degrees,
    Radians,
}

impl AngleUnit {
    fn to_radians(self, angle: Angle) -> Radians {
        match self {
            AngleUnit::Degrees => Radians::from_degrees_value(angle),
            AngleUnit::Radians => Radians::from_radians_value(angle),
        }
    }

    fn to_angle(self, angle: Radians) -> Angle {
        match self {
            AngleUnit::Degrees => angle.to_degrees(),
            AngleUnit::Radians => angle.to_radians(),
        }
    }
}

pub struct AsyncTurtle {
    client: ProtocolClient,
    id: TurtleId,
    angle_unit: AngleUnit,
}

impl From<Turtle> for AsyncTurtle {
    fn from(turtle: Turtle) -> Self {
        turtle.into_async()
    }
}

impl AsyncTurtle {
    pub async fn new() -> Self {
        let client = ProtocolClient::new().await
            .expect("unable to create renderer client");
        Self::with_client(client).await
    }

    /// Creates a new turtle using the given client
    pub(crate) async fn with_client(client: ProtocolClient) -> Self {
        let id = client.create_turtle().await
            .expect("unable to communicate with turtle server process");
        let angle_unit = AngleUnit::Degrees;

        Self {client, id, angle_unit}
    }

    pub async fn forward(&mut self, distance: Distance) {
        if !distance.is_normal() {
            return;
        }

        self.client.move_forward(self.id, distance).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn backward(&mut self, distance: Distance) {
        if !distance.is_normal() {
            return;
        }

        // Moving backwards is essentially moving forwards with a negative distance
        self.client.move_forward(self.id, -distance).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn right(&mut self, angle: Angle) {
        if !angle.is_normal() {
            return;
        }

        let angle = self.angle_unit.to_radians(angle);
        self.client.rotate_in_place(self.id, angle, RotationDirection::Clockwise).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn left(&mut self, angle: Angle) {
        if !angle.is_normal() {
            return;
        }

        let angle = self.angle_unit.to_radians(angle);
        self.client.rotate_in_place(self.id, angle, RotationDirection::Counterclockwise).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn wait(&mut self, secs: f64) {
        // This method *needs* exclusive access (`&mut self`) since otherwise another thread could
        // potentially call another method on the same turtle while it is supposed to be waiting.

        if !secs.is_normal() {
            return;
        }

        time::delay_for(time::Duration::from_millis((secs * 1000.0) as u64)).await
    }

    fn into_sync(self) -> Turtle {
        self.into()
    }

    pub async fn drawing(&self) -> Drawing {
        todo!()
    }

    pub async fn speed(&self) -> Speed {
        self.client.turtle_speed(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn set_speed<S: Into<Speed>>(&mut self, speed: S) {
        self.client.turtle_set_speed(self.id, speed.into()).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn position(&self) -> Point {
        self.client.turtle_position(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn go_to<P: Into<Point>>(&mut self, position: P) {
        self.client.move_to(self.id, position.into()).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn set_x(&mut self, x: f64) {
        let Point {x: _, y} = self.position().await;
        self.go_to(Point {x, y}).await
    }

    pub async fn set_y(&mut self, y: f64) {
        let Point {x, y: _} = self.position().await;
        self.go_to(Point {x, y}).await
    }

    pub async fn home(&mut self) {
        self.client.move_to(self.id, Point::origin()).await
            .expect("unable to communicate with turtle server process");
        self.client.turtle_reset_heading(self.id).await
            .expect("unable to communicate with turtle server process");
    }

    pub async fn heading(&self) -> Angle {
        let heading = self.client.turtle_heading(self.id).await
            .expect("unable to communicate with turtle server process");
        self.angle_unit.to_angle(heading)
    }

    pub async fn set_heading(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);

        let heading = self.client.turtle_heading(self.id).await
            .expect("unable to communicate with turtle server process");
        // Find the amount we need to turn to reach the target heading based on our current heading
        let angle = angle - heading;
        // Normalize the angle to be between -180 and 179 so that we rotate as little as possible
        // Formula from: https://stackoverflow.com/a/24234924/551904
        let angle = angle - radians::TWO_PI * ((angle + radians::PI) / radians::TWO_PI).floor();

        self.client.rotate_in_place(self.id, angle, RotationDirection::Counterclockwise).await
            .expect("unable to communicate with turtle server process")
    }

    pub fn is_using_degrees(&self) -> bool {
        self.angle_unit == AngleUnit::Degrees
    }

    pub fn is_using_radians(&self) -> bool {
        self.angle_unit == AngleUnit::Radians
    }

    pub fn use_degrees(&mut self) {
        self.angle_unit = AngleUnit::Degrees;
    }

    pub fn use_radians(&mut self) {
        self.angle_unit = AngleUnit::Radians;
    }

    pub async fn is_pen_down(&self) -> bool {
        self.client.turtle_pen_is_enabled(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn pen_down(&mut self) {
        self.client.turtle_pen_set_is_enabled(self.id, true).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn pen_up(&mut self) {
        self.client.turtle_pen_set_is_enabled(self.id, false).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn pen_size(&self) -> f64 {
        self.client.turtle_pen_thickness(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn set_pen_size(&mut self, thickness: f64) {
        assert!(
            thickness >= 0.0 && thickness.is_finite(),
            "Invalid thickness: {}. The pen thickness must be greater than or equal to zero",
            thickness
        );

        self.client.turtle_pen_set_thickness(self.id, thickness).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn pen_color(&self) -> Color {
        self.client.turtle_pen_color(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn set_pen_color<C: Into<Color> + Copy + Debug>(&mut self, color: C) {
        let pen_color = color.into();
        assert!(
            pen_color.is_valid(),
            "Invalid color: {:?}. See the color module documentation for more information.",
            color
        );
        self.client.turtle_pen_set_color(self.id, pen_color).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn fill_color(&self) -> Color {
        self.client.turtle_fill_color(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn set_fill_color<C: Into<Color> + Copy + Debug>(&mut self, color: C) {
        let fill_color = color.into();
        assert!(
            fill_color.is_valid(),
            "Invalid color: {:?}. See the color module documentation for more information.",
            color
        );
        self.client.turtle_set_fill_color(self.id, fill_color).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn is_filling(&self) -> bool {
        self.client.turtle_is_filling(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn begin_fill(&mut self) {
        self.client.begin_fill(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn end_fill(&mut self) {
        self.client.end_fill(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn is_visible(&self) -> bool {
        self.client.turtle_is_visible(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn hide(&mut self) {
        self.client.turtle_set_is_visible(self.id, false).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn show(&mut self) {
        self.client.turtle_set_is_visible(self.id, true).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn reset(&mut self) {
        self.clear().await;
        self.client.reset_turtle(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn clear(&mut self) {
        self.client.clear_turtle(self.id).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn turn_towards<P: Into<Point>>(&mut self, target: P) {
        let target: Point = target.into();
        let position = self.position().await;

        // If the target is (approximately) on the turtle don't turn
        if (target - position).is_not_normal() {
            return;
        }

        let heading = self.client.turtle_heading(self.id).await
            .expect("unable to communicate with turtle server process");

        // Calculate the target angle to reach
        let angle = (target - position).atan2();
        let angle = Radians::from_radians_value(angle);
        // Calculate how much turning will be needed (angle - heading)
        // And clamp it make sure the turtle doesn't turn more than 360 degrees
        let angle = (angle - heading) % radians::TWO_PI;
        // Try to rotate as little as possible
        let angle = if angle.abs() > radians::PI {
            // Use signum to make sure the angle has the right sign
            // And the turtle turns the right way
            -angle.signum() * (radians::TWO_PI - angle.abs())
        } else {
            angle
        };

        self.client.rotate_in_place(self.id, angle, RotationDirection::Counterclockwise).await
            .expect("unable to communicate with turtle server process")
    }

    pub async fn wait_for_click(&mut self) {
        todo!()
    }
}
