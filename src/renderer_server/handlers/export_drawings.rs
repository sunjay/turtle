use std::path::Path;

use tokio::sync::Mutex;

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
    ExportFormat,
};
use crate::renderer_client::ClientId;

use super::super::access_control::{AccessControl, RequiredData, RequiredTurtles};
use super::super::renderer::{export, display_list::DisplayList};

pub(crate) async fn export_drawings(
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
