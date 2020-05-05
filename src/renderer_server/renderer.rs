pub mod display_list;
pub mod export;

use glutin::dpi::PhysicalSize;
use pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use pathfinder_color::ColorF;
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

use crate::color::RGB_MAX_VAL;
use crate::{Point, Color};
use display_list::{DrawPrim, DisplayList};

/// Converts a color from the representation in this crate to the one used in the renderer
fn convert_color(color: Color) -> ColorF {
    let Color {red, green, blue, alpha} = color;
    ColorF::new(
        (red / RGB_MAX_VAL) as f32,
        (green / RGB_MAX_VAL) as f32,
        (blue / RGB_MAX_VAL) as f32,
        alpha as f32,
    )
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
}

impl Renderer {
    /// Creates a new renderer with the given physical size in pixels
    pub fn new(draw_size: PhysicalSize<u32>) -> Self {
        let renderer = PathfinderRenderer::new(
            GLDevice::new(GLVersion::GL3, 0),
            &EmbeddedResourceLoader::new(),
            DestFramebuffer::full_window(vec2i(draw_size.width as i32, draw_size.height as i32)),
            // This background color will be overwritten during the first render
            RendererOptions { background_color: Some(ColorF::white()) },
        );

        Self {
            renderer,
            font_context: CanvasFontContext::from_system_source(),
            scene: SceneProxy::new(RayonExecutor),
        }
    }

    /// Draw the given primitives onto a canvas of the given size
    ///
    /// Size is passed in to ensure that it is up-to-date
    pub fn render(&mut self, display_list: &DisplayList, draw_size: PhysicalSize<u32>) {
        //TODO: Use background color from Drawing state
        let background_color = ColorF::white();

        //TODO: Potentially re-create renderer/etc. if draw_size has changed

        // Clear to background color
        self.renderer.set_options(RendererOptions { background_color: Some(background_color) });

        let mut canvas = Canvas::new(vec2f(draw_size.width as f32, draw_size.height as f32))
            .get_context_2d(self.font_context.clone());

        //TODO: A `Point` is in logical coordinates, whereas Vector2F is in screen coordinates
        //TODO: Draw primitives
        let mut path = Path2D::new();
        path.move_to(vec2f(50.0, 140.0));
        path.line_to(vec2f(150.0, 60.0));
        path.line_to(vec2f(250.0, 140.0));
        path.close_path();
        canvas.stroke_path(path);

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
