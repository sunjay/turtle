use glutin::event_loop::EventLoopProxy;
use tokio::sync::Mutex;

use super::HandlerError;
use super::super::{
    main::MainThreadAction,
    app::{TurtleId, TurtleDrawings},
    access_control::{AccessControl, RequiredData, RequiredTurtles},
    renderer::display_list::DisplayList,
};

pub(crate) async fn begin_fill(
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: &Mutex<EventLoopProxy<MainThreadAction>>,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {state: turtle, drawings, current_fill_polygon} = turtles.one_mut();

    // Ignore the request if we are already filling
    if current_fill_polygon.is_some() {
        return Ok(());
    }

    let mut display_list = display_list.lock().await;
    let poly_handle = display_list.push_polygon_start(turtle.position, turtle.fill_color);
    drawings.push(poly_handle);
    *current_fill_polygon = Some(poly_handle);

    event_loop.lock().await.send_event(MainThreadAction::Redraw)?;

    Ok(())
}

pub(crate) async fn end_fill(
    app_control: &AccessControl,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;

    let TurtleDrawings {current_fill_polygon, ..} = turtles.one_mut();

    // No need to add the turtle's current position to the polygon since it should already be there

    // Changes nothing if we weren't filling already
    *current_fill_polygon = None;

    Ok(())
}
