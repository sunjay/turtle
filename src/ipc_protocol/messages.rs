use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::{Color, Point, Speed, Event, Distance, Size};
use crate::renderer_server::{TurtleId, ExportError};
use crate::radians::Radians;

/// The different kinds of requests that can be sent from a client
///
/// Not all requests warrant a response. The response for each request is listed in its
/// documentation.
///
/// Each request is processed in order and will be held until all of its required resources are
/// available. For example, suppose a request needs access to a particular turtle in order to
/// perform an animation. If another request comes along to clear the image, that request will be
/// held until the animation is complete since it also requires access to that turtle. This
/// behaviour is important to ensure sequential consistency and avoid race conditions as requests
/// run concurrently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientRequest {
    /// Creates a new turtle and provides the `TurtleId` that can be used to control it
    ///
    /// Response: `ServerResponse::NewTurtle`
    CreateTurtle,

    /// Export the drawing in its current state to the given path using the given format
    ///
    /// Response: `ServerResponse::ExportComplete`
    Export(PathBuf, ExportFormat),

    /// Get the next event (if any are currently available)
    ///
    /// Response: `ServerResponse::Event`
    PollEvent,

    /// Get the given property of the drawing
    ///
    /// Response: `ServerResponse::DrawingProp`
    DrawingProp(DrawingProp),
    /// Set the given property of the drawing
    ///
    /// Response: N/A
    SetDrawingProp(DrawingPropValue),
    /// Resets the given property of the drawing
    ///
    /// Response: N/A
    ResetDrawingProp(DrawingProp),

    /// Get the given property of a turtle
    ///
    /// Response: `ServerResponse::TurtleProp`
    TurtleProp(TurtleId, TurtleProp),
    /// Set the given property of a turtle
    ///
    /// Response: N/A
    SetTurtleProp(TurtleId, TurtlePropValue),
    /// Resets the given property of a turtle
    ///
    /// Response: N/A
    ResetTurtleProp(TurtleId, TurtleProp),
    /// Resets all properties of a turtle
    ///
    /// Response: N/A
    ResetTurtle(TurtleId),

    /// Move a turtle forward by the given amount
    ///
    /// The turtle moves in the direction of its heading and is animated at its current speed. This
    /// may draw a line if the turtle's pen is down. It may also result in a change to the current
    /// fill if the turtle is currently filling a shape.
    ///
    /// The response to this request provides no additional information, but is necessary to ensure
    /// that animations that should be sequenced, can be sequenced correctly. Without a signal for
    /// when the animation is complete, there would be no way to know when to start the next one.
    ///
    /// Response: `ServerResponse::AnimationComplete`
    MoveForward(TurtleId, Distance),
    /// Move a turtle to the given position
    ///
    /// The turtle movement is animated at its current speed. This may draw a line if the turtle's
    /// pen is down. It may also result in a change to the current fill if the turtle is currently
    /// filling a shape.
    ///
    /// The response to this request provides no additional information, but is necessary to ensure
    /// that animations that should be sequenced, can be sequenced correctly. Without a signal for
    /// when the animation is complete, there would be no way to know when to start the next one.
    ///
    /// Response: `ServerResponse::AnimationComplete`
    MoveTo(TurtleId, Point),
    /// Rotate a turtle in place by the given angle in the given direction
    ///
    /// The turtle rotates with an animation at its current speed. It will not draw any line while
    /// rotating since it has not moved.
    ///
    /// The response to this request provides no additional information, but is necessary to ensure
    /// that animations that should be sequenced, can be sequenced correctly. Without a signal for
    /// when the animation is complete, there would be no way to know when to start the next one.
    ///
    /// Response: `ServerResponse::AnimationComplete`
    RotateInPlace(TurtleId, Radians, RotationDirection),

    /// Creates a fill polygon from a turtle's current position
    ///
    /// The color of the fill will always be consistent with the fill color property of the turtle.
    /// That is, if the color changes in the middle of filling, the entire filled area will change
    /// color. It does not matter what the fill color was when this request was first sent.
    ///
    /// If the turtle was already filling when this request was sent, this request is ignored.
    ///
    /// Response: N/A
    BeginFill(TurtleId),
    /// Completes a fill polygon at a turtle's current position
    ///
    /// No further points will be added to the polygon and it will remain the color that it was at
    /// the time that this request was sent.
    ///
    /// If the turtle was not filling when this request was sent, this request is ignored.
    ///
    /// Response: N/A
    EndFill(TurtleId),

    /// Clears all drawings created by a turtle
    ///
    /// If no ID is provided, all drawings from all turtles will be cleared.
    ///
    /// The cleared turtles will not move. None of their positions, headings, pens, or other
    /// properties will change. If a turtle was current filling, it will stop filling and its
    /// fill polygon will also be cleared. No properties on the drawing will change, including the
    /// background image, title, etc.
    ///
    /// Response: N/A
    Clear(Option<TurtleId>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerResponse {
    /// The ID of a newly created turtle, guaranteed to be unique
    NewTurtle(TurtleId),

    /// The result of the export, possibly an error if something went wrong
    ExportComplete(Result<(), ExportError>),

    /// The next event, if any
    ///
    /// A response of `None` only indicates that no events are *currently* available, NOT that no
    /// events will *ever* be available.
    Event(Option<Event>),

    /// The value of the given property of the drawing
    DrawingProp(DrawingPropValue),

    /// The value of the given property of a turtle
    TurtleProp(TurtleId, TurtlePropValue),

    /// An animation was completed for a given turtle
    AnimationComplete(TurtleId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    Svg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrawingProp {
    Title,
    Background,
    Center,
    Size,
    Width,
    Height,
    IsMaximized,
    IsFullscreen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawingPropValue {
    Title(String),
    Background(Color),
    Center(Point),
    Size(Size),
    Width(u32),
    Height(u32),
    IsMaximized(bool),
    IsFullscreen(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurtleProp {
    Pen(PenProp),
    FillColor,
    IsFilling,
    Position,
    PositionX,
    PositionY,
    Heading,
    Speed,
    IsVisible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurtlePropValue {
    Pen(PenPropValue),
    FillColor(Color),
    /// NOTE: Instead of using this with `SetTurtleProp`, use `BeginFill` and `EndFill` instead.
    IsFilling(bool),
    /// NOTE: Instead of using this with `SetTurtleProp`, use `MoveTo` instead.
    Position(Point),
    /// NOTE: Instead of using this with `SetTurtleProp`, use `MoveTo` instead.
    PositionX(f64),
    /// NOTE: Instead of using this with `SetTurtleProp`, use `MoveTo` instead.
    PositionY(f64),
    /// NOTE: Instead of using this with `SetTurtleProp`, use `RotateInPlace` instead.
    Heading(Radians),
    Speed(Speed),
    IsVisible(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenProp {
    IsEnabled,
    Thickness,
    Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PenPropValue {
    IsEnabled(bool),
    Thickness(f64),
    Color(Color),
}
