use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    app::{App, TurtleId, TurtleDrawings},
    renderer::display_list::DisplayList,
};

pub(crate) fn clear_all(
    app: &mut App,
    display_list: &mut DisplayList,
    event_loop: &EventLoopNotifier,
) -> Result<(), HandlerError> {
    display_list.clear();

    for turtle in app.turtles_mut() {
        let TurtleDrawings {state: _, drawings, current_fill_polygon, animation} = turtle;
        debug_assert!(animation.is_none(),
            "bug: cannot clear turtle while animation is playing");

        drawings.clear();
        *current_fill_polygon = None;
    }

    // Signal the main thread that the image has changed
    event_loop.request_redraw()?;

    Ok(())
}

pub(crate) fn clear_turtle(
    app: &mut App,
    display_list: &mut DisplayList,
    event_loop: &EventLoopNotifier,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let turtle = app.turtle_mut(id);

    let TurtleDrawings {state: _, drawings, current_fill_polygon, animation} = turtle;
    debug_assert!(animation.is_none(),
        "bug: cannot clear turtle while animation is playing");

    display_list.remove(drawings.iter().copied());
    drawings.clear();
    *current_fill_polygon = None;

    // Signal the main thread that the image has changed
    event_loop.request_redraw()?;

    Ok(())
}
