use std::thread;

use glutin::{
    GlProfile,
    GlRequest,
    ContextBuilder,
    dpi::LogicalSize,
    window::WindowBuilder,
    event_loop::{ControlFlow, EventLoop},
};

use crate::renderer::Renderer;

/// Run the renderer process in the current thread
///
/// This function must run in the main thread ONLY
pub fn main() {
    assert_main_thread();

    let event_loop = EventLoop::new();

    //TODO: Use title, width and height from Drawing state instead of these hard-coded values
    let title = "Turtle";
    let width = 800.0; // px
    let height = 600.0; // px

    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize {width, height});

    // Create an OpenGL 3.x context for Pathfinder to use
    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Latest)
        .with_gl_profile(GlProfile::Core)
        //TODO: .with_double_buffer(Some(true))
        .build_windowed(window_builder, &event_loop)
        .expect("bug: either could not create window or could not build OpenGL context");

    // Load OpenGL, and make the context current
    let gl_context = unsafe { gl_context.make_current().unwrap() };
    gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

    // Need to draw using the physical size in pixels, not the logical size
    let draw_size = gl_context.window().inner_size();
    let mut renderer = Renderer::new(draw_size);

    loop {
        let draw_size = gl_context.window().inner_size();
        renderer.render((/* TODO */), draw_size);
    }
}

fn assert_main_thread() {
    // This check isn't foolproof. Someone can always create a thread named "main".
    if thread::current().name().unwrap_or("") != "main" {
        // In order to maintain compatibility with MacOS, we need to make sure that windows are
        // only created on the main thread. We do this check on all platforms so that no one
        // can accidentally make a change that creates the window off of the main thread.
        unreachable!("bug: windows can only be created on the main thread");
    }
}
