pub mod display_list;
pub mod export;

use glutin::dpi::PhysicalSize;
use pathfinder_canvas::{Canvas, CanvasFontContext, Path2D, LineCap, LineJoin, FillRule};
use pathfinder_color::ColorU;
use pathfinder_geometry::vector::{vec2f, vec2i};
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

use super::coords::ScreenPoint;
use super::state::{DrawingState, TurtleState};

use display_list::{DisplayList, DrawPrim, Line, Polygon};

/// Converts a color from the representation in this crate to the one used in the renderer
#[cfg_attr(any(feature = "test", test), allow(dead_code))]
fn convert_color(color: Color) -> ColorU {
    let Color {red, green, blue, alpha} = color;
    ColorU {
        r: red.round() as u8,
        g: green.round() as u8,
        b: blue.round() as u8,
        a: (alpha * 255.0).round() as u8,
    }
}

/// A renderer that draws on the current OpenGL context
#[cfg_attr(any(feature = "test", test), allow(dead_code))]
pub struct Renderer {
    renderer: PathfinderRenderer<GLDevice>,
    font_context: CanvasFontContext,
    scene: SceneProxy,
    /// Information about DPI scaling: https://docs.rs/glutin/0.24.0/glutin/dpi/index.html
    dpi_scale: f64,
}

#[cfg_attr(any(feature = "test", test), allow(dead_code))]
impl Renderer {
    /// Creates a new renderer with the given physical size in pixels
    pub fn new(draw_size: PhysicalSize<u32>, dpi_scale: f64) -> Self {
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
            dpi_scale,
        }
    }

    /// Updates the scale factor used during rendering
    pub fn scale_factor(&self) -> f64 {
        self.dpi_scale
    }

    /// Updates the scale factor used during rendering
    pub fn set_scale_factor(&mut self, dpi_scale: f64) {
        self.dpi_scale = dpi_scale;
    }

    /// Draw the given primitives onto a canvas of the given size
    ///
    /// Size is passed in to ensure that it is up-to-date
    pub fn render<'a>(
        &mut self,
        draw_size: PhysicalSize<u32>,
        display_list: &DisplayList,
        drawing: &DrawingState,
        turtles: impl Iterator<Item=&'a TurtleState>
    ) {
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
        let fb_center = (fb_size / 2.0).into();
        for prim in display_list.iter() {
            match prim {
                &DrawPrim::Line(Line {start, end, thickness, color}) => {
                    let mut path = Path2D::new();

                    path.move_to(ScreenPoint::from_logical(start, dpi_scale, center, fb_center).into());
                    path.line_to(ScreenPoint::from_logical(end, dpi_scale, center, fb_center).into());

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

                    path.move_to(ScreenPoint::from_logical(points[0], dpi_scale, center, fb_center).into());
                    for &point in &points[1..] {
                        path.line_to(ScreenPoint::from_logical(point, dpi_scale, center, fb_center).into());
                    }

                    path.close_path();

                    canvas.set_fill_style(convert_color(fill_color));
                    canvas.fill_path(path, FillRule::Winding);
                },
            }
        }

        // The turtle shell specified in logical coordinates relative to the turtle position
        let shell = &[Point {x: 0.0, y: 15.0}, Point {x: 10.0, y: 0.0}, Point {x: 0.0, y: -15.0}];
        for turtle in turtles {
            let &TurtleState {position, heading, is_visible, ..} = turtle;
            if !is_visible {
                continue;
            }

            let Point {x: turtle_x, y: turtle_y} = position;
            let cos = heading.cos();
            let sin = heading.sin();
            let shell_screen_coord = |Point {x, y}| {
                // Rotate each point by the heading and add the current turtle position
                let point = Point {
                    x: cos * x - sin * y + turtle_x,
                    y: sin * x + cos * y + turtle_y,
                };
                ScreenPoint::from_logical(point, dpi_scale, center, fb_center).into()
            };

            let mut path = Path2D::new();
            path.move_to(shell_screen_coord(shell[0]));
            for &point in &shell[1..] {
                path.line_to(shell_screen_coord(point));
            }
            path.close_path();
            canvas.set_fill_style(ColorU::white());
            canvas.fill_path(path.clone(), FillRule::Winding);
            canvas.set_line_width((1.0 * dpi_scale) as f32);
            canvas.set_stroke_style(ColorU::black());
            canvas.stroke_path(path);
        }

        // Build and render scene
        self.scene.replace_scene(canvas.into_canvas().into_scene());
        self.scene.build_and_render(&mut self.renderer, BuildOptions::default());
    }
}
