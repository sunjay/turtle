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
use tokio::{
    sync::Mutex,
    runtime::{Runtime, Handle},
};

use crate::ipc_protocol::ServerConnection;

use super::{
    RequestRedraw,
    app::App,
    renderer::{
        Renderer,
        display_list::DisplayList,
    },
};

/// Run the renderer process in the current thread
///
/// This function must run in the main thread ONLY
pub fn main() {
    assert_main_thread();

    // The state of the drawing and the state/drawings associated with each turtle
    let app = Arc::new(App::default());
    // All of the drawing primitives in the order in which they wil be drawn
    //
    // This is managed separately from the rest of the app state because the display list is shared
    // among pretty much everything and so critical sections containing the display list need to be
    // as short as possible.
    let display_list = Arc::new(Mutex::new(DisplayList::default()));

    let event_loop = EventLoop::with_user_event();

    // Spawn the actual server thread(s) that will handle incoming IPC messages and asynchronous
    // update the shared state
    let event_loop_proxy = event_loop.create_proxy();
    spawn_async_server(app.clone(), display_list.clone(), event_loop_proxy);

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
        .with_vsync(true)
        .with_double_buffer(Some(true))
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

        GlutinEvent::WindowEvent {
            event: WindowEvent::ScaleFactorChanged {scale_factor, ..},
            ..
        } => {
            renderer.set_scale_factor(scale_factor);
            //TODO: No idea if this next line is necessary or not
            gl_context.window().request_redraw();
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
            let handle = Handle::current();

            let drawing = {
                // Hold the drawing lock for as little time as possible
                let drawing = handle.block_on(app.drawing_mut());
                drawing.clone()
            };
            let display_list = handle.block_on(display_list.lock());

            let draw_size = gl_context.window().inner_size();
            renderer.render(draw_size, &display_list, &drawing);
            gl_context.swap_buffers().expect("unable to swap the buffer (for double buffering)");
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

fn spawn_async_server(
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: EventLoopProxy<RequestRedraw>,
) {
    thread::spawn(move || {
        let mut runtime = Runtime::new()
            .expect("unable to spawn tokio runtime to run turtle async server");

        // Spawn root task
        runtime.block_on(async {
            let conn = ServerConnection::connect_stdin().await
                .expect("unable to establish turtle server connection");
            super::serve(conn, app, display_list, event_loop).await;
        });
    });
}
