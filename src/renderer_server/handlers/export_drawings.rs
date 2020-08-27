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

    #[cfg(debug_assertions)]
    for turtle in app.turtles() {
        debug_assert!(turtle.animation.is_none(),
            "bug: cannot export while a turtle animation is playing");
    }

    use ExportFormat::*;
    let res = match format {
        Svg => export::save_svg(display_list, drawing, path),
    };

    conn.send(ServerResponse::ExportComplete(res))?;

    Ok(())
}
