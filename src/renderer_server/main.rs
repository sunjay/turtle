use std::thread;
use std::sync::Arc;

use glutin::{
    GlProfile,
    GlRequest,
    ContextBuilder,
    dpi::LogicalSize,
    window::WindowBuilder,
    event::{
        Event as GlutinEvent,
        WindowEvent,
        DeviceEvent,
        KeyboardInput,
        VirtualKeyCode,
        ElementState,
    },
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
};
use tokio::runtime::{Runtime, Handle};

use super::{
    RendererServer,
    RequestRedraw,
    app::App,
    renderer::Renderer,
};

/// Run the renderer process in the current thread
///
/// This function must run in the main thread ONLY
pub fn main() {
    assert_main_thread();

    let app = Arc::new(App::default());

    let event_loop = EventLoop::with_user_event();

    // Spawn the actual server thread(s) that will handle incoming IPC messages and asynchronous
    // update the shared state
    let event_loop_proxy = event_loop.create_proxy();
    spawn_async_server(app.clone(), event_loop_proxy);

    let window_builder = {
        let handle = Handle::current();
        let drawing = handle.block_on(app.drawing_mut());
        WindowBuilder::new()
            .with_title(&drawing.title)
            .with_inner_size(LogicalSize {width: drawing.width, height: drawing.height})
    };

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

    event_loop.run(move |event, _, control_flow| match event {
        // Quit if the window is closed or if Esc is pressed and then released
        GlutinEvent::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } | GlutinEvent::WindowEvent {
            event: WindowEvent::Destroyed,
            ..
        } | GlutinEvent::DeviceEvent {
            event: DeviceEvent::Key(KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            }),
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        },

        GlutinEvent::WindowEvent {window_id, event} => {
            //TODO: Check if event modifies state and then send event
        },
        GlutinEvent::DeviceEvent {device_id, event} => {
            //TODO: Check if event modifies state and then send event
        },

        GlutinEvent::UserEvent(RequestRedraw) => {
            gl_context.window().request_redraw();
        },

        GlutinEvent::RedrawRequested(_) => {
            let draw_size = gl_context.window().inner_size();
            renderer.render((/* TODO */), draw_size);
        },

        _ => {},
    });
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

fn spawn_async_server(app: Arc<App>, event_loop: EventLoopProxy<RequestRedraw>) {
    thread::spawn(move || {
        let mut runtime = Runtime::new()
            .expect("unable to spawn tokio runtime to run turtle async server");

        // Spawn root task
        runtime.block_on(async {
            let mut server = RendererServer::new(app, event_loop).await
                .expect("unable to establish turtle server connection");
            server.serve().await;
        });
    });
}
