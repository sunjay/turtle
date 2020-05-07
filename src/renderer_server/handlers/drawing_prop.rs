use glutin::{dpi::LogicalSize, event_loop::EventLoopProxy};
use tokio::sync::Mutex;

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
    DrawingProp,
    DrawingPropValue,
};
use crate::renderer_client::ClientId;

use super::super::{
    main::MainThreadAction,
    state::DrawingState,
    access_control::{AccessControl, RequiredData},
};

pub(crate) async fn drawing_prop(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    prop: DrawingProp,
) {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }).await;

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

    conn.send(client_id, ServerResponse::DrawingProp(value)).await
        .expect("unable to send response to IPC client");
}

pub(crate) async fn set_drawing_prop(
    app_control: &AccessControl,
    event_loop: &Mutex<EventLoopProxy<MainThreadAction>>,
    prop_value: DrawingPropValue,
) {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }).await;

    let drawing = data.drawing_mut();

    modify_drawing(drawing, event_loop, prop_value).await;
}

pub(crate) async fn reset_drawing_prop(
    app_control: &AccessControl,
    event_loop: &Mutex<EventLoopProxy<MainThreadAction>>,
    prop: DrawingProp,
) {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }).await;

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
    }).await;
}

async fn modify_drawing(
    drawing: &mut DrawingState,
    event_loop: &Mutex<EventLoopProxy<MainThreadAction>>,
    prop_value: DrawingPropValue,
) {
    use DrawingPropValue::*;
    match prop_value {
        Title(title) => {
            drawing.title = title.clone();

            // Signal the main thread to change this property on the window
            event_loop.lock().await.send_event(MainThreadAction::SetTitle(title))
                .expect("bug: event loop closed before animation completed");
        },

        Background(background) => drawing.background = background,

        Center(center) => drawing.center = center,

        Size(crate::Size {width, height}) => {
            drawing.width = width;
            drawing.height = height;

            // Signal the main thread to change this property on the window
            event_loop.lock().await.send_event(MainThreadAction::SetSize(LogicalSize {
                width: width as u32,
                height: height as u32,
            })).expect("bug: event loop closed before animation completed");
        },

        Width(width) => {
            drawing.width = width;

            // Signal the main thread to change this property on the window
            event_loop.lock().await.send_event(MainThreadAction::SetSize(LogicalSize {
                width: width as u32,
                height: drawing.height as u32,
            })).expect("bug: event loop closed before animation completed");
        },

        Height(height) => {
            drawing.height = height;

            // Signal the main thread to change this property on the window
            event_loop.lock().await.send_event(MainThreadAction::SetSize(LogicalSize {
                width: drawing.width as u32,
                height: height as u32,
            })).expect("bug: event loop closed before animation completed");
        },

        IsMaximized(is_maximized) => {
            drawing.is_maximized = is_maximized;

            // Signal the main thread to change this property on the window
            event_loop.lock().await.send_event(MainThreadAction::SetIsMaximized(is_maximized))
                .expect("bug: event loop closed before animation completed");
        },

        IsFullscreen(is_fullscreen) => {
            drawing.is_fullscreen = is_fullscreen;

            // Signal the main thread to change this property on the window
            event_loop.lock().await.send_event(MainThreadAction::SetIsFullscreen(is_fullscreen))
                .expect("bug: event loop closed before animation completed");
        },
    }
}
