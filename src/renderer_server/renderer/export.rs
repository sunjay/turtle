use std::fmt::Write;
use std::path::Path as FilePath;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use svg::node::element::{Line, Polygon, Rectangle};

use crate::{Point, Color};

use super::display_list::{DisplayList, DrawPrim, Line as DrawLine, Polygon as DrawPolygon};
use super::super::state::DrawingState;

/// See documentation in `renderer::to_screen_coords`
fn to_screen_coords(point: Point, center: Point, image_center: (f64, f64)) -> (f64, f64) {
    let Point {x, y} = point;
    let Point {x: center_x, y: center_y} = center;
    let (image_center_x, image_center_y) = image_center;

    (
        (x - center_x) + image_center_x,
        -(y - center_y) + image_center_y,
    )
}

/// Converts a color to its RGBA color string (suitable for SVG)
fn rgba(color: Color) -> String {
    let Color {red, green, blue, alpha} = color;
    format!("rgba({}, {}, {}, {})", red as u8, green as u8, blue as u8, alpha)
}

/// Converts a value into a string with the unit "px"
fn px(value: f64) -> String {
    format!("{}px", value)
}

/// Converts a list of pairs into a space-separated list of comma-separated pairs
///
/// The list must be non-empty
fn pairs(mut items: impl Iterator<Item=(f64, f64)>) -> String {
    let (a, b) = items.next().expect("list must be non-empty");
    let mut out = format!("{},{}", a, b);

    for (a, b) in items {
        write!(out, " {},{}", a, b).expect("write to string cannot fail");
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
    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, drawing.width, drawing.height));

    // set background color - https://stackoverflow.com/a/11293812/9276882
    let background = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", rgba(drawing.background));
    document = document.add(background);

    let center = drawing.center;
    let image_center = (drawing.width as f64 / 2.0, drawing.height as f64 / 2.0);
    for prim in display_list.iter() {
        match prim {
            &DrawPrim::Line(DrawLine {start, end, thickness, color}) => {
                let (start_x, start_y) = to_screen_coords(start, center, image_center);
                let (end_x, end_y) = to_screen_coords(end, center, image_center);

                let line = Line::new()
                    .set("x1", start_x)
                    .set("y1", start_y)
                    .set("x2", end_x)
                    .set("y2", end_y)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("stroke", rgba(color))
                    .set("stroke-width", px(thickness));

                document = document.add(line);
            },

            &DrawPrim::Polygon(DrawPolygon {ref points, fill_color}) => {
                // Skip obviously degenerate polygons
                if points.len() <= 2 {
                    continue;
                }

                let points = points.iter().map(|&p| to_screen_coords(p, center, image_center));
                let polygon = Polygon::new()
                    .set("points", pairs(points))
                    .set("fill-rule", "nonzero")
                    .set("fill", rgba(fill_color));

                document = document.add(polygon);
            },
        }
    }

    svg::save(path, &document).map_err(|err| ExportError(err.to_string()))
}
