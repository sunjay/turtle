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

    pub async fn create_turtle(&self) -> Result<TurtleId, ipc_channel::Error> {
        self.client.send(ClientRequest::CreateTurtle).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::NewTurtle(id) => Ok(id),
            _ => unreachable!("bug: expected to receive `NewTurtle` in response to `CreateTurtle` request"),
        }
    }

    pub async fn export_svg(&self, path: PathBuf) -> Result<Result<(), ExportError>, ipc_channel::Error> {
        self.client.send(ClientRequest::Export(path, ExportFormat::Svg)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::ExportComplete(res) => Ok(res),
            _ => unreachable!("bug: expected to receive `ExportComplete` in response to `Export` request"),
        }
    }

    pub async fn next_event(&self) -> Result<Event, ipc_channel::Error> {
        self.client.send(ClientRequest::NextEvent).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::Event(event) => Ok(event),
            _ => unreachable!("bug: expected to receive `Event` in response to `NextEvent` request"),
        }
    }

    pub async fn drawing_title(&self) -> Result<String, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Title)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Title(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_background(&self) -> Result<Color, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Background)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Background(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_center(&self) -> Result<Point, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Center)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Center(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_size(&self) -> Result<Size, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Size)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Size(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_width(&self) -> Result<u32, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Width)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Width(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_height(&self) -> Result<u32, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::Height)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::Height(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_is_maximized(&self) -> Result<bool, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::IsMaximized)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::IsMaximized(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_is_fullscreen(&self) -> Result<bool, ipc_channel::Error> {
        self.client.send(ClientRequest::DrawingProp(DrawingProp::IsFullscreen)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::DrawingProp(DrawingPropValue::IsFullscreen(value)) => Ok(value),
            _ => unreachable!("bug: expected to receive `DrawingProp` in response to `DrawingProp` request"),
        }
    }

    pub async fn drawing_set_title(&self, value: String) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Title(value))).await
    }

    pub async fn drawing_set_background(&self, value: Color) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Background(value))).await
    }

    pub async fn drawing_set_center(&self, value: Point) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Center(value))).await
    }

    pub async fn drawing_set_size(&self, value: Size) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Size(value))).await
    }

    pub async fn drawing_set_width(&self, value: u32) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Width(value))).await
    }

    pub async fn drawing_set_height(&self, value: u32) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::Height(value))).await
    }

    pub async fn drawing_set_is_maximized(&self, value: bool) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::IsMaximized(value))).await
    }

    pub async fn drawing_set_is_fullscreen(&self, value: bool) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetDrawingProp(DrawingPropValue::IsFullscreen(value))).await
    }

    pub async fn drawing_reset_center(&self) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::ResetDrawingProp(DrawingProp::Center)).await
    }

    pub async fn drawing_reset_size(&self) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::ResetDrawingProp(DrawingProp::Size)).await
    }

    pub async fn turtle_pen_is_enabled(&self, id: TurtleId) -> Result<bool, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::IsEnabled))).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::IsEnabled(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_thickness(&self, id: TurtleId) -> Result<f64, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::Thickness))).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::Thickness(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_color(&self, id: TurtleId) -> Result<Color, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Pen(PenProp::Color))).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Pen(PenPropValue::Color(value))) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_fill_color(&self, id: TurtleId) -> Result<Color, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::FillColor)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::FillColor(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_is_filling(&self, id: TurtleId) -> Result<bool, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::IsFilling)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::IsFilling(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_position(&self, id: TurtleId) -> Result<Point, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Position)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Position(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_position_x(&self, id: TurtleId) -> Result<f64, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::PositionX)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::PositionX(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_position_y(&self, id: TurtleId) -> Result<f64, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::PositionY)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::PositionY(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_heading(&self, id: TurtleId) -> Result<Radians, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Heading)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Heading(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_speed(&self, id: TurtleId) -> Result<Speed, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::Speed)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::Speed(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_is_visible(&self, id: TurtleId) -> Result<bool, ipc_channel::Error> {
        self.client.send(ClientRequest::TurtleProp(id, TurtleProp::IsVisible)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::TurtleProp(recv_id, TurtlePropValue::IsVisible(value)) => {
                debug_assert_eq!(id, recv_id, "bug: received data for incorrect turtle");
                Ok(value)
            },
            _ => unreachable!("bug: expected to receive `TurtleProp` in response to `TurtleProp` request"),
        }
    }

    pub async fn turtle_pen_set_is_enabled(&self, id: TurtleId, value: bool) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Pen(PenPropValue::IsEnabled(value)))).await
    }

    pub async fn turtle_pen_set_thickness(&self, id: TurtleId, value: f64) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Pen(PenPropValue::Thickness(value)))).await
    }

    pub async fn turtle_pen_set_color(&self, id: TurtleId, value: Color) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Pen(PenPropValue::Color(value)))).await
    }

    pub async fn turtle_set_fill_color(&self, id: TurtleId, value: Color) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::FillColor(value))).await
    }

    pub async fn turtle_set_speed(&self, id: TurtleId, value: Speed) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::Speed(value))).await
    }

    pub async fn turtle_set_is_visible(&self, id: TurtleId, value: bool) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::SetTurtleProp(id, TurtlePropValue::IsVisible(value))).await
    }

    pub async fn turtle_reset_heading(&self, id: TurtleId) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::ResetTurtleProp(id, TurtleProp::Heading)).await
    }

    pub async fn reset_turtle(&self, id: TurtleId) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::ResetTurtle(id)).await
    }

    pub async fn move_forward(&self, id: TurtleId, distance: Distance) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::MoveForward(id, distance)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
                Ok(())
            },
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `MoveForward` request"),
        }
    }

    pub async fn move_to(&self, id: TurtleId, target: Point) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::MoveTo(id, target)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
                Ok(())
            },
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `MoveTo` request"),
        }
    }

    pub async fn rotate_in_place(&self, id: TurtleId, angle: Radians, direction: RotationDirection) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::RotateInPlace(id, angle, direction)).await?;

        let response = self.client.recv().await;
        match response {
            ServerResponse::AnimationComplete(recv_id) => {
                debug_assert_eq!(id, recv_id, "bug: notified of complete animation for incorrect turtle");
                Ok(())
            },
            _ => unreachable!("bug: expected to receive `AnimationComplete` in response to `RotateInPlace` request"),
        }
    }

    pub async fn begin_fill(&self, id: TurtleId) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::BeginFill(id)).await
    }

    pub async fn end_fill(&self, id: TurtleId) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::EndFill(id)).await
    }

    pub async fn clear(&self) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::Clear(None)).await
    }

    pub async fn clear_turtle(&self, id: TurtleId) -> Result<(), ipc_channel::Error> {
        self.client.send(ClientRequest::Clear(Some(id))).await
    }
}
