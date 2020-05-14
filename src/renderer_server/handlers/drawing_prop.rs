use tokio::sync::oneshot;

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
    DrawingProp,
    DrawingPropValue,
};
use crate::renderer_client::ClientId;

use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    state::DrawingState,
    access_control::{AccessControl, RequiredData},
};

pub(crate) async fn drawing_prop(
    data_req_queued: oneshot::Sender<()>,
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    prop: DrawingProp,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }, data_req_queued).await;

    let drawing = data.drawing_mut();

    use DrawingProp::*;
    let value = match prop {
        Title => DrawingPropValue::Title(drawing.title.clone()),
        Background => DrawingPropValue::Background(drawing.background),
        Center => DrawingPropValue::Center(drawing.center),
        Size => DrawingPropValue::Size(crate::Size {width: drawing.width, height: drawing.height}),
        Width => DrawingPropValue::Width(drawing.width),
        Height => DrawingPropValue::Height(drawing.height),
        IsMaximized => DrawingPropValue::IsMaximized(drawing.is_maximized),
        IsFullscreen => DrawingPropValue::IsFullscreen(drawing.is_fullscreen),
    };

    conn.send(client_id, ServerResponse::DrawingProp(value)).await?;

    Ok(())
}

pub(crate) async fn set_drawing_prop(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    event_loop: &EventLoopNotifier,
    prop_value: DrawingPropValue,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }, data_req_queued).await;

    let drawing = data.drawing_mut();

    modify_drawing(drawing, event_loop, prop_value).await
}

pub(crate) async fn reset_drawing_prop(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    event_loop: &EventLoopNotifier,
    prop: DrawingProp,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }, data_req_queued).await;

    let drawing = data.drawing_mut();

    use DrawingProp::*;
    modify_drawing(drawing, event_loop, match prop {
        Title => DrawingPropValue::Title(DrawingState::DEFAULT_TITLE.to_string()),
        Background => DrawingPropValue::Background(DrawingState::DEFAULT_BACKGROUND),
        Center => DrawingPropValue::Center(DrawingState::DEFAULT_CENTER),
        Size => DrawingPropValue::Size(crate::Size {
            width: DrawingState::DEFAULT_WIDTH,
            height: DrawingState::DEFAULT_HEIGHT,
        }),
        Width => DrawingPropValue::Width(DrawingState::DEFAULT_WIDTH),
        Height => DrawingPropValue::Height(DrawingState::DEFAULT_HEIGHT),
        IsMaximized => DrawingPropValue::IsMaximized(DrawingState::DEFAULT_IS_MAXIMIZED),
        IsFullscreen => DrawingPropValue::IsFullscreen(DrawingState::DEFAULT_IS_FULLSCREEN),
    }).await
}

async fn modify_drawing(
    drawing: &mut DrawingState,
    event_loop: &EventLoopNotifier,
    prop_value: DrawingPropValue,
) -> Result<(), HandlerError> {
    use DrawingPropValue::*;
    match prop_value {
        Title(title) => {
            drawing.title = title.clone();

            // Signal the main thread to change this property on the window
            event_loop.set_title(title).await?;
        },

        Background(background) => drawing.background = background,

        Center(center) => drawing.center = center,

        Size(crate::Size {width, height}) => {
            drawing.width = width;
            drawing.height = height;

            // Signal the main thread to change this property on the window
            event_loop.set_size((width, height)).await?;
        },

        Width(width) => {
            drawing.width = width;

            // Signal the main thread to change this property on the window
            event_loop.set_size((width, drawing.height)).await?;
        },

        Height(height) => {
            drawing.height = height;

            // Signal the main thread to change this property on the window
            event_loop.set_size((drawing.width, height)).await?;
        },

        IsMaximized(is_maximized) => {
            drawing.is_maximized = is_maximized;

            // Signal the main thread to change this property on the window
            event_loop.set_is_maximized(is_maximized).await?;
        },

        IsFullscreen(is_fullscreen) => {
            drawing.is_fullscreen = is_fullscreen;

            // Signal the main thread to change this property on the window
            event_loop.set_is_fullscreen(is_fullscreen).await?;
        },
    }

    Ok(())
}
