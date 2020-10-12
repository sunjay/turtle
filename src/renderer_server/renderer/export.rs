use std::fmt::Write;
use std::path::Path as FilePath;

use serde::{Deserialize, Serialize};
use svg::node::element::{Line, Polygon, Rectangle};
use thiserror::Error;

use crate::Color;

use super::super::{coords::ScreenPoint, state::DrawingState};
use super::display_list::{DisplayList, DrawPrim, Line as DrawLine, Polygon as DrawPolygon};

/// Converts a color to its RGBA color string (suitable for SVG)
fn rgba(color: Color) -> String {
    let Color {
        red,
        green,
        blue,
        alpha,
    } = color;
    format!(
        "rgba({}, {}, {}, {})",
        red as u8, green as u8, blue as u8, alpha
    )
}

/// Converts a value into a string with the unit "px"
fn px(value: f64) -> String {
    format!("{}px", value)
}

/// Converts a list of pairs into a space-separated list of comma-separated pairs
///
/// The list must be non-empty
fn pairs(mut items: impl Iterator<Item = ScreenPoint>) -> String {
    let first = items.next().expect("list must be non-empty");
    let mut out = format!("{},{}", first.x, first.y);

    for pt in items {
        write!(out, " {},{}", pt.x, pt.y).expect("write to string cannot fail");
    }

    out
}

/// An error produced while exporting the drawing
#[derive(Debug, Error, Serialize, Deserialize)]
#[error("{0}")]
pub struct ExportError(String);

pub fn save_svg(
    display_list: &DisplayList,
    drawing: &DrawingState,
    path: &FilePath,
) -> Result<(), ExportError> {
    let mut document = svg::Document::new().set("viewBox", (0, 0, drawing.width, drawing.height));

    // set background color - https://stackoverflow.com/a/11293812/9276882
    let background = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", rgba(drawing.background));
    document = document.add(background);

    let center = drawing.center;
    let image_center = ScreenPoint {
        x: drawing.width as f64 / 2.0,
        y: drawing.height as f64 / 2.0,
    };
    for prim in display_list.iter() {
        match prim {
            &DrawPrim::Line(DrawLine {
                start,
                end,
                thickness,
                color,
            }) => {
                let start = ScreenPoint::from_logical(start, 1.0, center, image_center);
                let end = ScreenPoint::from_logical(end, 1.0, center, image_center);

                let line = Line::new()
                    .set("x1", start.x)
                    .set("y1", start.y)
                    .set("x2", end.x)
                    .set("y2", end.y)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("stroke", rgba(color))
                    .set("stroke-width", px(thickness));

                document = document.add(line);
            }

            &DrawPrim::Polygon(DrawPolygon {
                ref points,
                fill_color,
            }) => {
                // Skip obviously degenerate polygons
                if points.len() <= 2 {
                    continue;
                }

                let points = points
                    .iter()
                    .map(|&p| ScreenPoint::from_logical(p, 1.0, center, image_center));
                let polygon = Polygon::new()
                    .set("points", pairs(points))
                    .set("fill-rule", "nonzero")
                    .set("fill", rgba(fill_color));

                document = document.add(polygon);
            }
        }
    }

    svg::save(path, &document).map_err(|err| ExportError(err.to_string()))
}
