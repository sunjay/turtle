mod state;
mod app;
mod access_control;
mod renderer;
mod main;
mod start;

pub(crate) use app::TurtleId;
pub(crate) use renderer::export::ExportError;
pub use start::start;

use std::sync::Arc;
use std::path::Path;

use glutin::event_loop::EventLoopProxy;
use tokio::{
    sync::Mutex,
    io::{self, AsyncBufReadExt},
};

use crate::ipc_protocol::{
    ServerConnection,
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
};
use crate::renderer_client::ClientId;

use app::{App, TurtleDrawings};
use access_control::{AccessControl, RequiredData, RequiredTurtles};
use renderer::{export, display_list::DisplayList};

/// A custom event used to tell the glutin event loop to redraw the window
#[derive(Debug, Clone, PartialEq, Eq)]
struct RequestRedraw;

/// Establishes a connection to the client by reading from stdin
async fn connect() -> Result<ServerConnection, ConnectionError> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);

    let mut oneshot_name = String::new();
    reader.read_line(&mut oneshot_name).await?;
    if oneshot_name.is_empty() {
        panic!("bug: unexpected EOF when reading oneshot server name");
    }

    // Remove the trailing newline
    assert_eq!(oneshot_name.pop(), Some('\n'));
    let conn = ServerConnection::connect(oneshot_name)?;

    Ok(conn)
}

/// Serves requests from the client forever
async fn serve(
    conn: ServerConnection,
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: EventLoopProxy<RequestRedraw>,
) -> ! {
    let conn = Arc::new(conn);
    let app_control = Arc::new(AccessControl::new(app).await);
    let event_loop = Arc::new(Mutex::new(event_loop));

    loop {
        let (client_id, request) = conn.recv().await
            .expect("unable to receive request from IPC client");

        // Each incoming request is given its own task configured specifically for each kind of
        // request. Having separate tasks allows requests that can run in parallel to do so.
        tokio::spawn(run_request(
            conn.clone(),
            client_id,
            app_control.clone(),
            display_list.clone(),
            event_loop.clone(),
            request,
        ));
    }
}

async fn run_request(
    conn: Arc<ServerConnection>,
    client_id: ClientId,
    app_control: Arc<AccessControl>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: Arc<Mutex<EventLoopProxy<RequestRedraw>>>,
    request: ClientRequest,
) {
    use ClientRequest::*;
    match request {
        CreateTurtle => {
            let id = app_control.add_turtle().await;
            conn.send(client_id, ServerResponse::NewTurtle(id)).await
                .expect("unable to send IPC response");
        },

        Export(path, format) => {
            export_drawings(&conn, client_id, &app_control, &display_list, &path, format).await;
        },

        NextEvent => {
            todo!()
        },

        DrawingProp(prop) => {
            drawing_prop(&conn, client_id, &app_control, prop).await
        },
        SetDrawingProp(prop_value) => {
            set_drawing_prop(&app_control, prop_value).await
        },

        TurtleProp(id, prop) => {
            turtle_prop(&conn, client_id, &app_control, id, prop).await
        },
        SetTurtleProp(id, prop_value) => {
            set_turtle_prop(&app_control, id, prop_value).await
        },

        _ => todo!()
    }
}

async fn export_drawings(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    path: &Path,
    format: ExportFormat,
) {
    // We need to lock everything to ensure that the export takes place in a sequentially
    // consistent way. We wouldn't want this to run while any lines are still being drawn.
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: Some(RequiredTurtles::All),
    }).await;

    // Wait to lock the display list until we actually have the data from the access controller
    let display_list = display_list.lock().await;

    use ExportFormat::*;
    let res = match format {
        Svg => export::save_svg(&display_list, data.drawing_mut(), path),
    };

    conn.send(client_id, ServerResponse::ExportComplete(res)).await
        .expect("unable to send response to IPC client");
}

async fn drawing_prop(
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

async fn set_drawing_prop(app_control: &AccessControl, prop_value: DrawingPropValue) {
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: None,
    }).await;

    let drawing = data.drawing_mut();

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

async fn turtle_prop(
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    id: TurtleId,
    prop: TurtleProp,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, ..} = turtles.one_mut();

    use TurtleProp::*;
    use PenProp::*;
    let value = match prop {
        Pen(IsEnabled) => TurtlePropValue::Pen(PenPropValue::IsEnabled(turtle.pen.is_enabled)),
        Pen(Thickness) => TurtlePropValue::Pen(PenPropValue::Thickness(turtle.pen.thickness)),
        Pen(Color) => TurtlePropValue::Pen(PenPropValue::Color(turtle.pen.color)),
        FillColor => TurtlePropValue::FillColor(turtle.fill_color),
        IsFilling => TurtlePropValue::IsFilling(turtle.is_filling),
        Position => TurtlePropValue::Position(turtle.position),
        PositionX => TurtlePropValue::PositionX(turtle.position.x),
        PositionY => TurtlePropValue::PositionY(turtle.position.y),
        Heading => TurtlePropValue::Heading(turtle.heading),
        Speed => TurtlePropValue::Speed(turtle.speed),
        IsVisible => TurtlePropValue::IsVisible(turtle.is_visible),
    };

    conn.send(client_id, ServerResponse::TurtleProp(id, value)).await
        .expect("unable to send response to IPC client");
}

async fn set_turtle_prop(
    app_control: &AccessControl,
    id: TurtleId,
    prop_value: TurtlePropValue,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, ..} = turtles.one_mut();

    use TurtlePropValue::*;
    use PenPropValue::*;
    match prop_value {
        Pen(IsEnabled(is_enabled)) => turtle.pen.is_enabled = is_enabled,
        Pen(Thickness(thickness)) => turtle.pen.thickness = thickness,
        Pen(Color(color)) => turtle.pen.color = color,
        FillColor(fill_color) => turtle.fill_color = fill_color,
        IsFilling(is_filling) => turtle.is_filling = is_filling,
        Position(position) => turtle.position = position,
        PositionX(x) => turtle.position.x = x,
        PositionY(y) => turtle.position.y = y,
        Heading(heading) => turtle.heading = heading,
        Speed(speed) => turtle.speed = speed,
        IsVisible(is_visible) => turtle.is_visible = is_visible,
    }
}
