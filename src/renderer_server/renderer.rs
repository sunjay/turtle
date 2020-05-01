pub mod display_list;

use glutin::dpi::PhysicalSize;
use pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use pathfinder_color::ColorF;
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
    pub fn render(&mut self, prims: (/* TODO */), draw_size: PhysicalSize<u32>) {
        //TODO: Use background color from Drawing state
        let background_color = ColorF::white();

        //TODO: Potentially re-create renderer/etc. if draw_size has changed

        // Clear to background color
        self.renderer.set_options(RendererOptions { background_color: Some(background_color) });

        let mut canvas = Canvas::new(vec2f(draw_size.width as f32, draw_size.height as f32))
            .get_context_2d(self.font_context.clone());

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
