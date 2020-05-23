use tokio::sync::{oneshot, Mutex};

use super::HandlerError;
use super::super::{
    event_loop_notifier::EventLoopNotifier,
    app::{TurtleId, TurtleDrawings},
    access_control::{AccessControl, RequiredData, RequiredTurtles},
    renderer::display_list::DisplayList,
};

pub(crate) async fn clear_all(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: &EventLoopNotifier,
) -> Result<(), HandlerError> {
    // We need to lock everything to ensure that the clear takes place in a sequentially
    // consistent way. We wouldn't want this to run while any lines are still being drawn.
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: Some(RequiredTurtles::All),
    }, data_req_queued).await;

    let mut turtles = data.turtles_mut().await;
    let turtles = turtles.all_mut();

    // Wait to lock the display list until we actually have the data from the access controller
    let mut display_list = display_list.lock().await;

    display_list.clear();
    for turtle in turtles {
        let TurtleDrawings {state: _, drawings, current_fill_polygon} = &mut **turtle;

        drawings.clear();
        *current_fill_polygon = None;
    }

    // Signal the main thread that the image has changed
    event_loop.request_redraw().await?;

    Ok(())
}

pub(crate) async fn clear_turtle(
    data_req_queued: oneshot::Sender<()>,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: &EventLoopNotifier,
    id: TurtleId,
) -> Result<(), HandlerError> {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }, data_req_queued).await;
    let mut turtles = data.turtles_mut().await;
    let TurtleDrawings {state: _, drawings, current_fill_polygon} = turtles.one_mut();

    // Wait to lock the display list until we actually have the data from the access controller
    let mut display_list = display_list.lock().await;

    display_list.remove(drawings.iter().copied());
    drawings.clear();
    *current_fill_polygon = None;

    // Signal the main thread that the image has changed
    event_loop.request_redraw().await?;

    Ok(())
}
