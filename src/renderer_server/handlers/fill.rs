use tokio::sync::{oneshot, Mutex};

use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    app::{TurtleId, TurtleDrawings},
    access_control::AccessControl,
    renderer::display_list::DisplayList,
};

pub(crate) async fn begin_fill(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: EventLoopNotifier,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let turtle = app_control.get(id, data_req_queued).await;
    let mut turtle = turtle.lock().await;

    let TurtleDrawings {state: turtle, drawings, current_fill_polygon} = &mut *turtle;

    // Ignore the request if we are already filling
    if current_fill_polygon.is_some() {
        return Ok(());
    }

    let mut display_list = display_list.lock().await;
    let poly_handle = display_list.push_polygon_start(turtle.position, turtle.fill_color);
    drawings.push(poly_handle);
    *current_fill_polygon = Some(poly_handle);

    event_loop.request_redraw()?;

    Ok(())
}

pub(crate) async fn end_fill(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let turtle = app_control.get(id, data_req_queued).await;
    let mut turtle = turtle.lock().await;

    let TurtleDrawings {current_fill_polygon, ..} = &mut *turtle;

    // No need to add the turtle's current position to the polygon since it should already be there

    // Changes nothing if we weren't filling already
    *current_fill_polygon = None;

    Ok(())
}
