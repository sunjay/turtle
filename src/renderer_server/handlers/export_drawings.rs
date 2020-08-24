use std::path::Path;

use tokio::sync::{oneshot, Mutex};

use crate::ipc_protocol::{ServerOneshotSender, ServerResponse, ExportFormat};

use super::HandlerError;
use super::super::{
    access_control::AccessControl,
    renderer::{export, display_list::DisplayList},
};

pub(crate) async fn export_drawings(
    data_req_queued: oneshot::Sender<()>,
    conn: ServerOneshotSender,
    app_control: &AccessControl,
    display_list: &Mutex<DisplayList>,
    path: &Path,
    format: ExportFormat,
) -> Result<(), HandlerError> {
    // We need to lock everything to ensure that the export takes place in a sequentially
    // consistent way. We wouldn't want this to run while any lines are still being drawn.
    let (drawing, _turtles) = app_control.get_all(data_req_queued).await;

    let drawing = drawing.lock().await;

    // Wait to lock the display list until we actually have the data from the access controller
    let display_list = display_list.lock().await;

    use ExportFormat::*;
    let res = match format {
        Svg => export::save_svg(&display_list, &drawing, path),
    };

    conn.send(ServerResponse::ExportComplete(res))?;

    Ok(())
}
