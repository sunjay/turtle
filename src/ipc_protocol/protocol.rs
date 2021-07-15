use std::path::PathBuf;

use crate::radians::Radians;
use crate::renderer_client::RendererClient;
use crate::renderer_server::{ExportError, TurtleId};
use crate::{async_turtle::AngleUnit, debug, Color, Distance, Event, Point, Size, Speed};

use super::{
    ClientRequest, ConnectionError, DrawingProp, DrawingPropValue, ExportFormat, PenProp, PenPropValue, RotationDirection, ServerResponse,
    TurtleProp, TurtlePropValue,
};

/// A wrapper for `RendererClient` that encodes the the IPC protocol in a type-safe manner
pub struct ProtocolClient {
    client: RendererClient,
}

impl From<RendererClient> for ProtocolClient {
    fn from(client: RendererClient) -> Self {
        Self { client }
    }
}

impl ProtocolClient {
    /// Spawns a new server process and creates a connection to it
    pub async fn new() -> Result<Self, ConnectionError> {
        let client = RendererClient::new().await?;
        Ok(client.into())
    }

    /// Creates a new renderer client that can also communicate to the same server
    pub async fn split(&self) -> Self {
        self.client.split().await.into()
    }

    pub async fn create_turtle(&self) -> TurtleId {
        self.client.send(ClientRequest::CreateTurtle);

        let response = self.client.recv().await;
        match response {
            ServerResponse::NewTurtle(id) => id,
            _ => unreachable!("bug: expected to receive `NewTurtle` in response to `CreateTurtle` request"),
        }
    }

    pub async fn export_svg(&self, path: PathBuf) -> Result<(), ExportError> {
        self.client.send(ClientRequest::Export(path, ExportFormat::Svg));

        let response = self.client.recv().await;
        match response {
            ServerResponse::ExportComplete(res) => res,
            _ => unreachable!("bug: expected to receive `ExportComplete` in response to `Export` request"),
        }
    }

    pub async fn poll_event(&self) -> Option<Event> {
        self.client.send(ClientRequest::PollEvent);

        let response = self.client.recv().await;
        match response {
            ServerResponse::Event(event) => event,
            _ => unreachable!("bug: expected to receive `Event` in response to `NextEvent` request"),
        }
    }

    pub async fn drawing_title(&self) -> String {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Title));

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Title(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_background(&self) -> Color {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Background));

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Background(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_center(&self) -> Point {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Center));

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Center(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_size(&self) -> Size {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Size));

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Size(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_is_maximized(&self) -> bool {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::IsMaximized));

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::IsMaximized(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_is_fullscreen(&self) -> bool {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::IsFullscreen));

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::IsFullscreen(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub fn drawing_set_title(&self, value: String) {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Title(value)))
    }

    pub fn drawing_set_background(&self, value: Color) {
        debug_assert!(
            value.is_valid(),
            "bug: colors should be validated before sending to renderer server"
        );
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Background(value)))
    }

    pub fn drawing_set_center(&self, value: Point) {
        debug_assert!(
            value.is_finite(),
            "bug: center should be validated before sending to renderer server"
        );
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Center(value)))
    }

    pub fn drawing_set_size(&self, value: Size) {
        debug_assert!(
            value.width > 0 && value.height > 0,
            "bug: size should be validated before sending to renderer server"
        );
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Size(value)))
    }

    pub fn drawing_set_is_maximized(&self, value: bool) {
        self.client
            .send(ClientRequest::SetDrawingProp(DrawingPropValue::IsMaximized(value)))
    }

    pub fn drawing_set_is_fullscreen(&self, value: bool) {
        self.client
            .send(ClientRequest::SetDrawingProp(DrawingPropValue::IsFullscreen(value)))
    }

    pub fn drawing_reset_center(&self) {
        self.client.send(ClientRequest::ResetDrawingProp(DrawingProp::Center))
    }

    pub fn drawing_reset_size(&self) {
        self.client.send(ClientRequest::ResetDrawingProp(DrawingProp::Size))
    }

    pub async fn turtle_pen_is_enabled(&self, id: TurtleId) -> bool {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::IsEnabled)));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::IsEnabled(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_thickness(&self, id: TurtleId) -> f64 {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::Thickness)));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::Thickness(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_color(&self, id: TurtleId) -> Color {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::Color)));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::Color(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_fill_color(&self, id: TurtleId) -> Color {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::FillColor));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::FillColor(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_is_filling(&self, id: TurtleId) -> bool {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::IsFilling));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::IsFilling(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_position(&self, id: TurtleId) -> Point {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Position));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Position(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_heading(&self, id: TurtleId) -> Radians {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Heading));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Heading(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_speed(&self, id: TurtleId) -> Speed {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Speed));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Speed(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_is_visible(&self, id: TurtleId) -> bool {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::IsVisible));

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::IsVisible(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            }
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub fn turtle_pen_set_is_enabled(&self, id: TurtleId, value: bool) {
        self.client.send(ClientRequest::SetTurtleProp(
            id,
            TurtlePropValue::Pen(PenPropValue::IsEnabled(value)),
        ))
    }

    pub fn turtle_pen_set_thickness(&self, id: TurtleId, value: f64) {
        debug_assert!(
            value >= 0.0 && value.is_finite(),
            "bug: pen size should be validated before sending to renderer server"
        );
        self.client.send(ClientRequest::SetTurtleProp(
            id,
            TurtlePropValue::Pen(PenPropValue::Thickness(value)),
        ))
    }

    pub fn turtle_pen_set_color(&self, id: TurtleId, value: Color) {
        debug_assert!(
            value.is_valid(),
            "bug: colors should be validated before sending to renderer server"
        );
        self.client
            .send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Pen(PenPropValue::Color(value))))
    }

    pub fn turtle_set_fill_color(&self, id: TurtleId, value: Color) {
        debug_assert!(
            value.is_valid(),
            "bug: colors should be validated before sending to renderer server"
        );
        self.client
            .send(ClientRequest::SetTurtleProp(id, TurtlePropValue::FillColor(value)))
    }

    pub fn turtle_set_speed(&self, id: TurtleId, value: Speed) {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Speed(value)))
    }

    pub fn turtle_set_is_visible(&self, id: TurtleId, value: bool) {
        self.client
            .send(ClientRequest::SetTurtleProp(id, TurtlePropValue::IsVisible(value)))
    }

    pub fn turtle_reset_heading(&self, id: TurtleId) {
        self.client.send(ClientRequest::ResetTurtleProp(id, TurtleProp::Heading))
    }

    pub fn reset_turtle(&self, id: TurtleId) {
        self.client.send(ClientRequest::ResetTurtle(id))
    }

    pub async fn move_forward(&self, id: TurtleId, distance: Distance) {
        if !distance.is_normal() {
            return;
        }

        self.client.send(ClientRequest::MoveForward(id, distance));

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
            }
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `MoveForward` request"),
        }
    }

    pub async fn move_to(&self, id: TurtleId, target: Point) {
        if !target.is_finite() {
            return;
        }

        self.client.send(ClientRequest::MoveTo(id, target));

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
            }
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `MoveTo` request"),
        }
    }

    pub async fn rotate_in_place(&self, id: TurtleId, angle: Radians, direction: RotationDirection) {
        if !angle.is_normal() {
            return;
        }

        self.client.send(ClientRequest::RotateInPlace(id, angle, direction));

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
            }
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `RotateInPlace` request"),
        }
    }

    pub async fn circular_arc(&self, id: TurtleId, radius: Distance, extent: Radians, direction: RotationDirection) {
        if !radius.is_normal() || !extent.is_normal() {
            return;
        }

        let steps = 250; // Arbitrary value for now.
        let step = radius.abs() * extent.to_radians() / steps as f64;
        let rotation = radius.signum() * extent / steps as f64;

        for _ in 0..steps {
            self.move_forward(id, step).await;
            self.rotate_in_place(id, rotation, direction).await;
        }
    }

    pub fn begin_fill(&self, id: TurtleId) {
        self.client.send(ClientRequest::BeginFill(id))
    }

    pub fn end_fill(&self, id: TurtleId) {
        self.client.send(ClientRequest::EndFill(id))
    }

    pub fn clear_all(&self) {
        self.client.send(ClientRequest::ClearAll)
    }

    pub fn clear_turtle(&self, id: TurtleId) {
        self.client.send(ClientRequest::ClearTurtle(id))
    }

    pub async fn debug_turtle(&self, id: TurtleId, angle_unit: AngleUnit) -> debug::Turtle {
        self.client.send(ClientRequest::DebugTurtle(id, angle_unit));

        let response = self.client.recv().await;
        match response {
            ServerResponse::DebugTurtle(recv_id, state) => {
                debug_assert_eq!(id, recv_id, "bug: received debug turtle for incorrect turtle");
                state
            }
            _ => unreachable!("bug: expected to receive `DebugTurtle` in response to `DebugTurtle` request"),
        }
    }

    pub async fn debug_drawing(&self) -> debug::Drawing {
        self.client.send(ClientRequest::DebugDrawing);

        let response = self.client.recv().await;
        match response {
            ServerResponse::DebugDrawing(state) => state,
            _ => unreachable!("bug: expected to receive `DebugDrawing` in response to `DebugDrawing` request"),
        }
    }

    pub fn destroy(self) {
        self.client.send(ClientRequest::DestroyDrawing);
    }
}
