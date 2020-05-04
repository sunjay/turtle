use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
    DrawingProp,
    DrawingPropValue,
};
use crate::renderer_client::ClientId;

use super::super::access_control::{AccessControl, RequiredData};

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

pub(crate) async fn set_drawing_prop(app_control: &AccessControl, prop_value: DrawingPropValue) {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }).await;

    let drawing = data.drawing_mut();

    //TODO: Send `RequestRedraw` since many of these change the image
    //TODO: Send events through EventLoopProxy that indicate changes in the window (e.g. for
    // changes to `is_maximized` we should call the appropriate method on the Window)
    use DrawingPropValue::*;
    match prop_value {
        Title(title) => drawing.title = title,
        Background(background) => drawing.background = background,
        Center(center) => drawing.center = center,
        Size(crate::Size {width, height}) => {
            drawing.width = width;
            drawing.height = height;
        },
        Width(width) => drawing.width = width,
        Height(height) => drawing.height = height,
        IsMaximized(is_maximized) => drawing.is_maximized = is_maximized,
        IsFullscreen(is_fullscreen) => drawing.is_fullscreen = is_fullscreen,
    }
}
