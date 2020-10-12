use super::super::{
    app::{App, TurtleDrawings, TurtleId},
    event_loop_notifier::EventLoopNotifier,
    renderer::display_list::DisplayList,
};
use super::HandlerError;

pub(crate) fn begin_fill(
    app: &mut App,
    display_list: &mut DisplayList,
    event_loop: &EventLoopNotifier,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let turtle = app.turtle_mut(id);

    let TurtleDrawings {
        state: turtle,
        drawings,
        current_fill_polygon,
    } = turtle;

    // Ignore the request if we are already filling
    if current_fill_polygon.is_some() {
        return Ok(());
    }

    let poly_handle = display_list.push_polygon_start(turtle.position, turtle.fill_color);
    drawings.push(poly_handle);
    *current_fill_polygon = Some(poly_handle);

    event_loop.request_redraw()?;

    Ok(())
}

pub(crate) fn end_fill(app: &mut App, id: TurtleId) -> Result<(), HandlerError> {
    let turtle = app.turtle_mut(id);

    let TurtleDrawings {
        current_fill_polygon,
        ..
    } = turtle;

    // No need to add the turtle's current position to the polygon since it should already be there

    // Changes nothing if we weren't filling already
    *current_fill_polygon = None;

    Ok(())
}
