use std::path::PathBuf;

use crate::renderer_client::RendererClient;
use crate::renderer_server::{TurtleId, ExportError};
use crate::radians::Radians;
use crate::{Distance, Point, Color, Speed, Event, Size};

use super::{
    ConnectionError,
    ClientRequest,
    ServerResponse,
    ExportFormat,
    DrawingProp,
    DrawingPropValue,
    TurtleProp,
    TurtlePropValue,
    PenProp,
    PenPropValue,
    RotationDirection,
};

/// A wrapper for `RendererClient` that encodes the the IPC protocol in a type-safe manner
pub struct ProtocolClient {
    client: RendererClient,
}

impl From<RendererClient> for ProtocolClient {
    fn from(client: RendererClient) -> Self {
        Self {client}
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
        self.client.send(ClientRequest::CreateTurtle).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::NewTurtle(id) => id,
            _ => unreachable!("bug: expected to receive `NewTurtle` in response to `CreateTurtle` request"),
        }
    }

    pub async fn export_svg(&self, path: PathBuf) -> Result<(), ExportError> {
        self.client.send(ClientRequest::Export(path, ExportFormat::Svg)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::ExportComplete(res) => res,
            _ => unreachable!("bug: expected to receive `ExportComplete` in response to `Export` request"),
        }
    }

    pub async fn next_event(&self) -> Event {
        self.client.send(ClientRequest::NextEvent).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::Event(event) => event,
            _ => unreachable!("bug: expected to receive `Event` in response to `NextEvent` request"),
        }
    }

    pub async fn drawing_title(&self) -> String {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Title)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Title(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_background(&self) -> Color {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Background)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Background(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_center(&self) -> Point {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Center)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Center(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_size(&self) -> Size {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Size)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Size(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_is_maximized(&self) -> bool {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::IsMaximized)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::IsMaximized(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_is_fullscreen(&self) -> bool {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::IsFullscreen)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::IsFullscreen(value)) => value,
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_set_title(&self, value: String) {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Title(value))).await
    }

    pub async fn drawing_set_background(&self, value: Color) {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Background(value))).await
    }

    pub async fn drawing_set_center(&self, value: Point) {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Center(value))).await
    }

    pub async fn drawing_set_size(&self, value: Size) {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Size(value))).await
    }

    pub async fn drawing_set_is_maximized(&self, value: bool) {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::IsMaximized(value))).await
    }

    pub async fn drawing_set_is_fullscreen(&self, value: bool) {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::IsFullscreen(value))).await
    }

    pub async fn drawing_reset_center(&self) {
        self.client.send(ClientRequest::ResetDrawingProp(DrawingProp::Center)).await
    }

    pub async fn drawing_reset_size(&self) {
        self.client.send(ClientRequest::ResetDrawingProp(DrawingProp::Size)).await
    }

    pub async fn turtle_pen_is_enabled(&self, id: TurtleId) -> bool {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::IsEnabled))).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::IsEnabled(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_thickness(&self, id: TurtleId) -> f64 {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::Thickness))).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::Thickness(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_color(&self, id: TurtleId) -> Color {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::Color))).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::Color(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_fill_color(&self, id: TurtleId) -> Color {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::FillColor)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::FillColor(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_is_filling(&self, id: TurtleId) -> bool {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::IsFilling)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::IsFilling(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_position(&self, id: TurtleId) -> Point {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Position)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Position(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_heading(&self, id: TurtleId) -> Radians {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Heading)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Heading(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_speed(&self, id: TurtleId) -> Speed {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Speed)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Speed(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_is_visible(&self, id: TurtleId) -> bool {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::IsVisible)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::IsVisible(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                value
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_set_is_enabled(&self, id: TurtleId, value: bool) {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Pen(PenPropValue::IsEnabled(value)))).await
    }

    pub async fn turtle_pen_set_thickness(&self, id: TurtleId, value: f64) {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Pen(PenPropValue::Thickness(value)))).await
    }

    pub async fn turtle_pen_set_color(&self, id: TurtleId, value: Color) {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Pen(PenPropValue::Color(value)))).await
    }

    pub async fn turtle_set_fill_color(&self, id: TurtleId, value: Color) {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::FillColor(value))).await
    }

    pub async fn turtle_set_speed(&self, id: TurtleId, value: Speed) {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Speed(value))).await
    }

    pub async fn turtle_set_is_visible(&self, id: TurtleId, value: bool) {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::IsVisible(value))).await
    }

    pub async fn turtle_reset_heading(&self, id: TurtleId) {
        self.client.send(ClientRequest::ResetTurtleProp(id, TurtleProp::Heading)).await
    }

    pub async fn reset_turtle(&self, id: TurtleId) {
        self.client.send(ClientRequest::ResetTurtle(id)).await
    }

    pub async fn move_forward(&self, id: TurtleId, distance: Distance) {
        self.client.send(ClientRequest::MoveForward(id, distance)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
            },
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `MoveForward` request"),
        }
    }

    pub async fn move_to(&self, id: TurtleId, target: Point) {
        self.client.send(ClientRequest::MoveTo(id, target)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
            },
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `MoveTo` request"),
        }
    }

    pub async fn rotate_in_place(&self, id: TurtleId, angle: Radians, direction: RotationDirection) {
        self.client.send(ClientRequest::RotateInPlace(id, angle, direction)).await;

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
            },
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `RotateInPlace` request"),
        }
    }

    pub async fn begin_fill(&self, id: TurtleId) {
        self.client.send(ClientRequest::BeginFill(id)).await
    }

    pub async fn end_fill(&self, id: TurtleId) {
        self.client.send(ClientRequest::EndFill(id)).await
    }

    #[allow(dead_code)] //TODO(#16): This is part of the multiple turtles feature (for Drawing::clear())
    pub async fn clear(&self) {
        self.client.send(ClientRequest::Clear(None)).await
    }

    pub async fn clear_turtle(&self, id: TurtleId) {
        self.client.send(ClientRequest::Clear(Some(id))).await
    }
}
