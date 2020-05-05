pub mod display_list;
pub mod export;

use glutin::dpi::PhysicalSize;
use pathfinder_canvas::{Canvas, CanvasFontContext, Path2D, LineCap, LineJoin, FillRule};
use pathfinder_color::ColorU;
use pathfinder_geometry::vector::{vec2f, vec2i, Vector2F};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use pathfinder_renderer::{
    concurrent::rayon::RayonExecutor,
    concurrent::scene_proxy::SceneProxy,
    options::BuildOptions,
    gpu::{
        renderer::Renderer as PathfinderRenderer,
        options::{DestFramebuffer, RendererOptions},
    },
};

use crate::{Point, Color};

use super::state::DrawingState;

use display_list::{DisplayList, DrawPrim, Line, Polygon};

/// Converts a color from the representation in this crate to the one used in the renderer
fn convert_color(color: Color) -> ColorU {
    let Color {red, green, blue, alpha} = color;
    ColorU {
        r: red.round() as u8,
        g: green.round() as u8,
        b: blue.round() as u8,
        a: (alpha * 255.0).round() as u8,
    }
}

/// Converts a `Point` in logical or "world" coordinates to a `Vector2F` in screen coordinates
///
/// # Parameters
///
/// * `point` is the `Point` to convert to screen coordinates
/// * `dpi_scale` is the high DPI scale factor (>= 0.0)
/// * `center` is the `Point` configured in the drawing that all other `Point`s are relative to.
/// * `fb_center` is the center of the framebuffer in screen coordinates.
///
/// # Coordinate Systems
///
/// * logical or "world" coordinates (cartesian coordinates)
///   * origin is in the center of the framebuffer and can be offset by `center`
///   * x is left to right
///   * y is bottom to top
/// * screen coordinates
///   * origin is the top-left corner of the framebuffer
///   * x is left to right
///   * y is top to bottom
fn to_screen_coords(point: Point, dpi_scale: f64, center: Point, fb_center: Vector2F) -> Vector2F {
    let Point {x, y} = point;
    let Point {x: center_x, y: center_y} = center;
    let fb_center_x = fb_center.x();
    let fb_center_y = fb_center.y();

    vec2f(
        ((x - center_x) * dpi_scale) as f32 + fb_center_x,
        -((y - center_y) * dpi_scale) as f32 + fb_center_y,
    )
}

/// A renderer that draws on the current OpenGL context
pub struct Renderer {
    renderer: PathfinderRenderer<GLDevice>,
    font_context: CanvasFontContext,
    scene: SceneProxy,
    /// Information about DPI scaling: https://docs.rs/glutin/0.24.0/glutin/dpi/index.html
    dpi_scale: f64,
}

impl Renderer {
    /// Creates a new renderer with the given physical size in pixels
    pub fn new(draw_size: PhysicalSize<u32>) -> Self {
        let renderer = PathfinderRenderer::new(
            GLDevice::new(GLVersion::GL3, 0),
            &EmbeddedResourceLoader::new(),
            DestFramebuffer::full_window(vec2i(draw_size.width as i32, draw_size.height as i32)),
            // This background color will be overwritten during the first render
            RendererOptions {
                background_color: Some(ColorU::white().to_f32()),
                ..RendererOptions::default()
            },
        );

        Self {
            renderer,
            font_context: CanvasFontContext::from_system_source(),
            scene: SceneProxy::new(RayonExecutor),
            // Default is 1.0 according to: https://docs.rs/glutin/0.24.0/glutin/dpi/index.html#events
            dpi_scale: 1.0,
        }
    }

    /// Updates the scale factor used during rendering
    pub fn set_scale_factor(&mut self, dpi_scale: f64) {
        self.dpi_scale = dpi_scale;
    }

    /// Draw the given primitives onto a canvas of the given size
    ///
    /// Size is passed in to ensure that it is up-to-date
    pub fn render(&mut self, draw_size: PhysicalSize<u32>, display_list: &DisplayList, drawing: &DrawingState) {
        // Set the current draw size
        self.renderer.replace_dest_framebuffer(
            DestFramebuffer::full_window(vec2i(draw_size.width as i32, draw_size.height as i32))
        );

        // Clear to background color
        self.renderer.set_options(RendererOptions {
            background_color: Some(convert_color(drawing.background).to_f32()),
            ..RendererOptions::default()
        });

        // The size of the framebuffer
        let fb_size = vec2f(draw_size.width as f32, draw_size.height as f32);
        let mut canvas = Canvas::new(fb_size)
            .get_context_2d(self.font_context.clone());

        // Set default options for all operations
        canvas.set_line_cap(LineCap::Round);
        canvas.set_line_join(LineJoin::Round);

        //TODO: Remove this line once servo/pathfinder#318 is fixed.
        //  Link: https://github.com/servo/pathfinder/issues/318
        // Need to render *something* every time to get pathfinder to even render a background
        canvas.stroke_rect(pathfinder_geometry::rect::RectF::new(vec2f(0.0, 0.0), vec2f(1.0, 1.0)));

        // Draw each primitive
        let dpi_scale = self.dpi_scale;
        let center = drawing.center;
        let fb_center = fb_size / 2.0;
        for prim in display_list.iter() {
            match prim {
                &DrawPrim::Line(Line {start, end, thickness, color}) => {
                    let mut path = Path2D::new();

                    path.move_to(to_screen_coords(start, dpi_scale, center, fb_center));
                    path.line_to(to_screen_coords(end, dpi_scale, center, fb_center));

                    canvas.set_line_width((thickness * dpi_scale) as f32);
                    canvas.set_stroke_style(convert_color(color));
                    canvas.stroke_path(path);
                },

                &DrawPrim::Polygon(Polygon {ref points, fill_color}) => {
                    // Skip obviously degenerate polygons
                    if points.len() <= 2 {
                        continue;
                    }

                    let mut path = Path2D::new();

                    path.move_to(to_screen_coords(points[0], dpi_scale, center, fb_center));
                    for &point in &points[1..] {
                        path.line_to(to_screen_coords(point, dpi_scale, center, fb_center));
                    }

                    path.close_path();

                    canvas.set_fill_style(convert_color(fill_color));
                    canvas.fill_path(path, FillRule::Winding);
                },
            }
        }

        // Build and render scene
        self.scene.replace_scene(canvas.into_canvas().into_scene());
        self.scene.build_and_render(&mut self.renderer, BuildOptions::default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_coords() {
        // The origin is always at fb_center as long as center is also the origin
        let screen_coord = to_screen_coords(Point::origin(), 1.0, Point::origin(), vec2f(200.0, 300.0));
        assert_eq!(screen_coord, vec2f(200.0, 300.0));
        // The origin is always at fb_center regardless of DPI scale
        let screen_coord = to_screen_coords(Point::origin(), 2.0, Point::origin(), vec2f(200.0, 300.0));
        assert_eq!(screen_coord, vec2f(200.0, 300.0));

        // The x-axis and y-axis treated distinctly and interpreted as cartesian
        let screen_coord = to_screen_coords(Point {x: 10.0, y: 20.0}, 1.0, Point::origin(), vec2f(200.0, 300.0));
        assert_eq!(screen_coord, vec2f(210.0, 280.0));
        // A different fb_center gives a different final coordinate
        let screen_coord = to_screen_coords(Point {x: 10.0, y: 20.0}, 1.0, Point::origin(), vec2f(300.0, 400.0));
        assert_eq!(screen_coord, vec2f(310.0, 380.0));

        // The center is interpreted as cartesian and points are relative to it
        let screen_coord = to_screen_coords(Point {x: 10.0, y: 20.0}, 1.0, Point {x: 30.0, y: 5.0}, vec2f(200.0, 300.0));
        assert_eq!(screen_coord, vec2f(180.0, 285.0));

        // Negative points work too
        let screen_coord = to_screen_coords(Point {x: -10.0, y: -20.0}, 1.0, Point {x: 30.0, y: -5.0}, vec2f(200.0, 300.0));
        assert_eq!(screen_coord, vec2f(160.0, 315.0));

        // DPI scale > 1.0 causes logical coordinates to scale, but NOT screen coordinates
        let screen_coord = to_screen_coords(Point {x: 10.0, y: 20.0}, 2.0, Point {x: 30.0, y: 5.0}, vec2f(200.0, 300.0));
        assert_eq!(screen_coord, vec2f(160.0, 270.0));
    }
}
