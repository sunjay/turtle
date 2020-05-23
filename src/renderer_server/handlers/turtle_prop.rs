use tokio::sync::{oneshot, Mutex};

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
    TurtleProp,
    TurtlePropValue,
    PenProp,
    PenPropValue,
};
use crate::renderer_client::ClientId;

use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    state::{self, TurtleState},
    app::{TurtleId, TurtleDrawings},
    access_control::{AccessControl, RequiredData, RequiredTurtles},
    renderer::display_list::DisplayList,
};

pub(crate) async fn turtle_prop(
    data_req_queued: oneshot::Sender<()>,
    conn: &ServerConnection,
    client_id: ClientId,
    app_control: &AccessControl,
    id: TurtleId,
    prop: TurtleProp,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }, data_req_queued).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, current_fill_polygon, ..} = turtles.one_mut();

    use TurtleProp::*;
    use PenProp::*;
    let value = match prop {
        Pen(IsEnabled) => TurtlePropValue::Pen(PenPropValue::IsEnabled(turtle.pen.is_enabled)),
        Pen(Thickness) => TurtlePropValue::Pen(PenPropValue::Thickness(turtle.pen.thickness)),
        Pen(Color) => TurtlePropValue::Pen(PenPropValue::Color(turtle.pen.color)),
        FillColor => TurtlePropValue::FillColor(turtle.fill_color),
        IsFilling => TurtlePropValue::IsFilling(current_fill_polygon.is_some()),
        Position => TurtlePropValue::Position(turtle.position),
        PositionX => TurtlePropValue::PositionX(turtle.position.x),
        PositionY => TurtlePropValue::PositionY(turtle.position.y),
        Heading => TurtlePropValue::Heading(turtle.heading),
        Speed => TurtlePropValue::Speed(turtle.speed),
        IsVisible => TurtlePropValue::IsVisible(turtle.is_visible),
    };

    conn.send(client_id, ServerResponse::TurtleProp(id, value)).await?;

    Ok(())
}

pub(crate) async fn set_turtle_prop(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: EventLoopNotifier,
    id: TurtleId,
    prop_value: TurtlePropValue,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }, data_req_queued).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, current_fill_polygon, ..} = turtles.one_mut();

    use TurtlePropValue::*;
    use PenPropValue::*;
    match prop_value {
        Pen(IsEnabled(is_enabled)) => turtle.pen.is_enabled = is_enabled,
        Pen(Thickness(thickness)) => turtle.pen.thickness = thickness,
        Pen(Color(color)) => turtle.pen.color = color,

        FillColor(fill_color) => {
            turtle.fill_color = fill_color;

            // Update the current fill polygon to the new color
            if let Some(poly_handle) = *current_fill_polygon {
                let mut display_list = display_list.lock().await;
                display_list.polygon_set_fill_color(poly_handle, fill_color);

                // Signal the main thread that the image has changed
                event_loop.request_redraw()?;
            }
        },

        IsFilling(_) => unreachable!("bug: should have used `BeginFill` and `EndFill` instead"),
        Position(_) |
        PositionX(_) |
        PositionY(_) => unreachable!("bug: should have used `MoveTo` instead"),
        Heading(_) => unreachable!("bug: should have used `RotateInPlace` instead"),

        Speed(speed) => turtle.speed = speed,

        IsVisible(is_visible) => {
            turtle.is_visible = is_visible;

            // Signal the main thread that the image has changed
            event_loop.request_redraw()?;
        },
    }

    Ok(())
}

pub(crate) async fn reset_turtle_prop(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: EventLoopNotifier,
    id: TurtleId,
    prop: TurtleProp,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }, data_req_queued).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, current_fill_polygon, ..} = turtles.one_mut();

    let mut drawing_changed = false;

    use TurtleProp::*;
    use PenProp::*;
    match prop {
        Pen(IsEnabled) => turtle.pen.is_enabled = state::Pen::DEFAULT_IS_ENABLED,
        Pen(Thickness) => turtle.pen.thickness = state::Pen::DEFAULT_THICKNESS,
        Pen(Color) => turtle.pen.color = state::Pen::DEFAULT_COLOR,

        FillColor => {
            turtle.fill_color = TurtleState::DEFAULT_FILL_COLOR;

            // Update the current fill polygon to the new color
            if let Some(poly_handle) = *current_fill_polygon {
                let mut display_list = display_list.lock().await;
                display_list.polygon_set_fill_color(poly_handle, TurtleState::DEFAULT_FILL_COLOR);

                drawing_changed = true;
            }
        },

        IsFilling => unreachable!("bug: should have used `BeginFill` and `EndFill` instead"),

        Position => {
            turtle.position = TurtleState::DEFAULT_POSITION;
            drawing_changed = true;
        },
        PositionX => {
            turtle.position.x = TurtleState::DEFAULT_POSITION.x;
            drawing_changed = true;
        },
        PositionY => {
            turtle.position.y = TurtleState::DEFAULT_POSITION.y;
            drawing_changed = true;
        },

        Heading => {
            turtle.heading = TurtleState::DEFAULT_HEADING;
            drawing_changed = true;
        },

        Speed => turtle.speed = crate::Speed::default(),

        IsVisible => {
            turtle.is_visible = TurtleState::DEFAULT_IS_VISIBLE;
            drawing_changed = true;
        },
    }

    if drawing_changed {
        // Signal the main thread that the image has changed
        event_loop.request_redraw()?;
    }

    Ok(())
}

pub(crate) async fn reset_turtle(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: EventLoopNotifier,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }, data_req_queued).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, current_fill_polygon, ..} = turtles.one_mut();

    *turtle = TurtleState::default();

    // Update the current fill polygon to the new color
    if let Some(poly_handle) = *current_fill_polygon {
        let mut display_list = display_list.lock().await;
        display_list.polygon_set_fill_color(poly_handle, TurtleState::DEFAULT_FILL_COLOR);
    }

    // Signal the main thread that the image has changed
    event_loop.request_redraw()?;

    Ok(())
}
