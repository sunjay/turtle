use std::path::Path;

use crate::ipc_protocol::{ServerOneshotSender, ServerResponse, ExportFormat};

use super::HandlerError;
use super::super::{
    app::App,
    renderer::{export, display_list::DisplayList},
};

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
        Png => export::save_png(display_list, drawing, path),
    };

    conn.send(ServerResponse::ExportComplete(res))?;

    Ok(())
}
