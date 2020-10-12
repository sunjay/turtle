use std::path::Path;

use crate::ipc_protocol::{ExportFormat, ServerOneshotSender, ServerResponse};

use super::super::{
    app::App,
    renderer::{display_list::DisplayList, export},
};
use super::HandlerError;

pub(crate) fn export_drawings(
    conn: ServerOneshotSender,
    app: &App,
    display_list: &DisplayList,
    path: &Path,
    format: ExportFormat,
) -> Result<(), HandlerError> {
    let drawing = app.drawing();

    use ExportFormat::*;
    let res = match format {
        Svg => export::save_svg(display_list, drawing, path),
    };

    conn.send(ServerResponse::ExportComplete(res))?;

    Ok(())
}
