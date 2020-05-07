use glutin::event_loop::EventLoopProxy;
use tokio::sync::Mutex;

use super::super::{
    main::MainThreadAction,
    app::{TurtleId, TurtleDrawings},
    access_control::{AccessControl, RequiredData, RequiredTurtles},
    renderer::display_list::DisplayList,
};

pub(crate) async fn clear(
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: &Mutex<EventLoopProxy<MainThreadAction>>,
) {
    // We need to lock everything to ensure that the clear takes place in a sequentially
    // consistent way. We wouldn't want this to run while any lines are still being drawn.
    let mut data = app_control.get(RequiredData {
        drawing: true,
        turtles: Some(RequiredTurtles::All),
    }).await;

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
    event_loop.lock().await.send_event(MainThreadAction::Redraw)
        .expect("bug: event loop closed before animation completed");
}

pub(crate) async fn clear_turtle(
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    event_loop: &Mutex<EventLoopProxy<MainThreadAction>>,
    id: TurtleId,
) {
    let mut data = app_control.get(RequiredData {
        drawing: false,
        turtles: Some(RequiredTurtles::One(id)),
    }).await;
    let mut turtles = data.turtles_mut().await;
    let TurtleDrawings {state: _, drawings, current_fill_polygon} = turtles.one_mut();

    // Wait to lock the display list until we actually have the data from the access controller
    let mut display_list = display_list.lock().await;

    display_list.remove(drawings.iter().copied());
    drawings.clear();
    *current_fill_polygon = None;

    // Signal the main thread that the image has changed
    event_loop.lock().await.send_event(MainThreadAction::Redraw)
        .expect("bug: event loop closed before animation completed");
}
