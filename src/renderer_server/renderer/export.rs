use std::fmt::Write;
use std::path::Path as FilePath;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use svg::node::element::{Line, Polygon, Rectangle};
use image;
use nsvg;

use crate::Color;

use super::display_list::{DisplayList, DrawPrim, Line as DrawLine, Polygon as DrawPolygon};
use super::super::{
    coords::ScreenPoint,
    state::DrawingState,
};

/// Converts a color to its RGB color string (suitable for SVG)
fn rgb(color: Color) -> String {
    let Color {red, green, blue, alpha: _} = color;
    format!("rgb({}, {}, {})", red as u8, green as u8, blue as u8)
}

/// Converts a value into a string with the unit "px"
fn px(value: f64) -> String {
    format!("{}px", value)
}

/// Converts a list of pairs into a space-separated list of comma-separated pairs
///
/// The list must be non-empty
fn pairs(mut items: impl Iterator<Item=ScreenPoint>) -> String {
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

fn create_svg_document(
    display_list: &DisplayList,
    drawing: &DrawingState,
) -> Result<svg::Document, ExportError> {
    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, drawing.width, drawing.height));

    // set background color - https://stackoverflow.com/a/11293812/9276882
    let background = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill-opacity", drawing.background.alpha)
        .set("fill", rgb(drawing.background));
    document = document.add(background);

    let center = drawing.center;
    let image_center = ScreenPoint {
        x: drawing.width as f64 / 2.0,
        y: drawing.height as f64 / 2.0,
    };
    for prim in display_list.iter() {
        match prim {
            &DrawPrim::Line(DrawLine {start, end, thickness, color}) => {
                let start = ScreenPoint::from_logical(start, 1.0, center, image_center);
                let end = ScreenPoint::from_logical(end, 1.0, center, image_center);

                let line = Line::new()
                    .set("x1", start.x)
                    .set("y1", start.y)
                    .set("x2", end.x)
                    .set("y2", end.y)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("stroke", rgb(color))
                    .set("stroke-opacity", color.alpha)
                    .set("stroke-width", px(thickness));

                document = document.add(line);
            },

            &DrawPrim::Polygon(DrawPolygon {ref points, fill_color}) => {
                // Skip obviously degenerate polygons
                if points.len() <= 2 {
                    continue;
                }

                let points = points.iter()
                    .map(|&p| ScreenPoint::from_logical(p, 1.0, center, image_center));
                let polygon = Polygon::new()
                    .set("points", pairs(points))
                    .set("fill-rule", "nonzero")
                    .set("fill-opacity", fill_color.alpha)
                    .set("fill", rgb(fill_color));

                document = document.add(polygon);
            },
        }
    }

    Ok(document)
}


pub fn save_svg(
    display_list: &DisplayList,
    drawing: &DrawingState,
    path: &FilePath,
) -> Result<(), ExportError> {
    let document = create_svg_document(display_list, drawing)?;
    svg::save(path, &document).map_err(|err| ExportError(err.to_string()))
}

pub fn save_png(
    display_list: &DisplayList,
    drawing: &DrawingState,
    path: &FilePath,
) -> Result<(), ExportError> {
    let document = create_svg_document(display_list, drawing)?;

    // Some SVG features are not supported by nsvg:
    //    - Text elements are ignored, although text can simply be converted to a path and it will work just fine
    //    - Embedded bitmap images are ignored
    //    - Scripts are ignored
    //    - Animations are ignored
    // If this becomes an issue we can "usvg" to convert the svg into a svg of just paths.
    let svg = nsvg::parse_str(&document.to_string(), nsvg::Units::Pixel, 96.0)
        .map_err(|err| ExportError(err.to_string()))?;

    // Rasterize the loaded SVG and return an RgbaImage
    let image = svg.rasterize(1.0).map_err(|err| ExportError(err.to_string()))?;

    let (width, height) = image.dimensions();

    // Write the image to disk as a PNG
    image::save_buffer(
        path,
        &image.into_raw(),
        width,
        height,
        image::ColorType::Rgba8,
    ).map_err(|err| ExportError(err.to_string()))
}