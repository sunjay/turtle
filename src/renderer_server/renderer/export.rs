use std::path::Path;

use thiserror::Error;
use serde::{Serialize, Deserialize};

use super::display_list::DisplayList;
use super::super::state::DrawingState;

/// An error produced while exporting the drawing
#[derive(Debug, Error, Serialize, Deserialize)]
#[error("{0}")]
pub struct ExportError(String);

pub fn save_svg(
    display_list: &DisplayList,
    drawing: &DrawingState,
    path: &Path,
) -> Result<(), ExportError> {
    todo!()
}
