use std::time::{Instant, Duration};
use std::future::Future;

use glutin::{
    GlProfile,
    GlRequest,
    ContextBuilder,
    WindowedContext,
    PossiblyCurrent,
    dpi::{LogicalSize, PhysicalPosition},
    window::{WindowBuilder, Fullscreen},
    event::{
        Event as GlutinEvent,
        StartCause,
        WindowEvent,
        KeyboardInput,
        VirtualKeyCode,
        ElementState,
    },
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};
use tokio::{
    sync::mpsc,
    runtime::Handle,
};

use crate::Event;
use crate::ipc_protocol::{ServerSender, ServerReceiver, ConnectionError};

use super::{
    app::{SharedApp, App},
    coords::ScreenPoint,
    renderer::{
        Renderer,
        display_list::{SharedDisplayList, DisplayList},
    },
    event_loop_notifier::{EventLoopNotifier, MainThreadAction},
};

/// The maximum rendering FPS allowed
///
/// Rendering is intentionally throttled to avoid too much contention over the display list. If
/// multiple turtles are used or if many lines are drawn quickly, we may get >= 1 redraw request
/// per *millisecond* this is far too many redraws. Limiting to this rate helps avoid that.
const MAX_RENDERING_FPS: u64 = 60;

// 1,000,000 us in 1 s
const MICROS_PER_SEC: u64 = 1_000_000;

fn new_event_loop<T>() -> EventLoop<T> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            EventLoop::with_user_event()

        } else if #[cfg(target_os = "windows")] {
            use glutin::platform::windows::EventLoopExtWindows;
            EventLoop::new_any_thread()

        } else {
            use glutin::platform::unix::EventLoopExtUnix;
            EventLoop::new_any_thread()
        }
    }
}

/// Run the window event loop in the current thread/task
///
/// When the window is opened, this will spawn a task that establishes the server connection using
/// the given future and begin to serve client requests.
///
/// On some platforms this function may only run on the main thread
pub fn run_main(
    // A handle used to block on asynchronous code and spawn new tasks
    //
    // Necessary because this function is not run on a runtime thread in all backends
    handle: Handle,

    // Polled to establish the server connection
    establish_connection: impl Future<Output=Result<(ServerSender, ServerReceiver), ConnectionError>> + Send + 'static,
) {
    // The state of the drawing and the state/drawings associated with each turtle
    let app = SharedApp::default();
    // All of the drawing primitives in the order in which they wil be drawn
    //
    // Critical sections containing the display list should be as short as possible to avoid holding
    // up the renderer.
    let display_list = SharedDisplayList::default();

    let mut event_loop = new_event_loop();
    // Create the proxy that will be given to the thread managing IPC
    let event_loop_notifier = EventLoopNotifier::new(event_loop.create_proxy());

    // A channel for transferring events
    let (events_sender, events_receiver) = mpsc::unbounded_channel();
    // Put these variables in an Option so we can call `take()` in the event loop. Required
    // because borrow checker cannot verify which events only fire once.
    let mut events_receiver = Some(events_receiver);
    let mut establish_connection = Some(establish_connection);
    // Using a bounded (size = 1) channel because a oneshot consumes self when awaited and this
    // needs to be polled multiple times
    let (mut server_shutdown, server_shutdown_receiver) = mpsc::channel(1);
    let mut server_shutdown_receiver = Some(server_shutdown_receiver);

    let window_builder = {
        let app = app.read();
        let drawing = app.drawing();
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
        .expect("either could not create window or could not build OpenGL context");

    // Load OpenGL, and make the context current
    let gl_context = unsafe { gl_context.make_current().unwrap() };
    gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

    // Need to draw using the physical size in pixels, not the logical size
    let draw_size = gl_context.window().inner_size();
    let mut renderer = Renderer::new(draw_size, gl_context.window().scale_factor());

    // For rate limiting rendering
    let min_render_delay = Duration::from_micros(MICROS_PER_SEC / MAX_RENDERING_FPS);
    // Subtracting the delay so we do an initial render right away
    let mut last_render = Instant::now() - min_render_delay;
    // Very important to use `run_return` here instead of `run` because `run` calls process::exit()
    // and that is not appropriate for the multithreaded backend as that would cause the entire
    // process to end when the window is closed.
    event_loop.run_return(move |event, _, control_flow| match event {
        GlutinEvent::NewEvents(StartCause::Init) => {
            // Spawn the actual server thread(s) that will handle incoming IPC messages and
            // asynchronous update the shared state
            //
            // Note that putting this code here instead of before the event loop causes the
            // `Turtle::new()`, etc. methods not to return before the window opens. Those methods
            // can't return because the connection handshake cannot complete before the thread used
            // for IPC is spawned.
            spawn_async_server(
                &handle,
                app.clone(),
                display_list.clone(),
                event_loop_notifier.clone(),
                events_receiver.take().expect("bug: init event should only occur once"),
                establish_connection.take().expect("bug: init event should only occur once"),
                server_shutdown_receiver.take().expect("bug: init event should only occur once"),
            );
        },

        GlutinEvent::NewEvents(StartCause::ResumeTimeReached {..}) => {
            // A render was delayed in the `RedrawRequested` so let's try to do it again now that
            // we have resumed
            gl_context.window().request_redraw();
        },

        // Quit if the window is closed or if Esc is pressed and then released
        GlutinEvent::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } | GlutinEvent::WindowEvent {
            event: WindowEvent::Destroyed,
            ..
        } | GlutinEvent::WindowEvent {
            event: WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            },
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        },

        GlutinEvent::WindowEvent {
            event: WindowEvent::ScaleFactorChanged {scale_factor, ..},
            ..
        } => {
            renderer.set_scale_factor(scale_factor);
        },

        GlutinEvent::WindowEvent {event, ..} => {
            let scale_factor = renderer.scale_factor();
            match event {
                WindowEvent::Resized(size) => {
                    let size = size.to_logical(scale_factor);
                    let mut app = app.write();
                    let mut drawing = app.drawing_mut();
                    drawing.width = size.width;
                    drawing.height = size.height;
                },

                //TODO: There are currently no events for updating is_maximized, so that property
                // should not be relied on. https://github.com/rust-windowing/glutin/issues/1298

                _ => {},
            }

            // Converts to logical coordinates, only locking the drawing if this is actually called
            let to_logical = |pos: PhysicalPosition<f64>| {
                let app = app.read();
                let drawing = app.drawing();
                let center = drawing.center;
                let draw_size = gl_context.window().inner_size();
                let fb_center = ScreenPoint {
                    x: draw_size.width as f64 / 2.0,
                    y: draw_size.height as f64 / 2.0,
                };

                let pos: ScreenPoint = pos.into();
                pos.to_logical(scale_factor, center, fb_center)
            };

            //TODO: There is no guarantee that sending this event here will actually allow a client
            // to receive it. After all, if the window closes and this process exits, there will be
            // no way to handle subsequent `NextEvent` requests.
            if let Some(event) = Event::from_window_event(event, scale_factor, to_logical) {
                // Sending may fail if the IPC thread has ended due to a disconnection when the
                // main process ends. This is not a fatal error though so we just ignore it.
                events_sender.send(event).unwrap_or(());
            }
        },

        // Window events are currently sufficient for the turtle event API
        GlutinEvent::DeviceEvent {..} => {},

        GlutinEvent::UserEvent(MainThreadAction::Redraw) => {
            gl_context.window().request_redraw();
        },

        GlutinEvent::UserEvent(MainThreadAction::SetTitle(title)) => {
            gl_context.window().set_title(&title);
        },

        GlutinEvent::UserEvent(MainThreadAction::SetSize(size)) => {
            gl_context.window().set_inner_size(size);
        },

        GlutinEvent::UserEvent(MainThreadAction::SetIsMaximized(is_maximized)) => {
            gl_context.window().set_maximized(is_maximized);
        },

        GlutinEvent::UserEvent(MainThreadAction::SetIsFullscreen(is_fullscreen)) => {
            gl_context.window().set_fullscreen(if is_fullscreen {
                Some(Fullscreen::Borderless(gl_context.window().current_monitor()))
            } else { None });
        },

        GlutinEvent::RedrawRequested(_) => {
            // Check if we just rendered
            let last_render_delay = last_render.elapsed();
            if last_render_delay < min_render_delay {
                let remaining = min_render_delay - last_render_delay;
                *control_flow = ControlFlow::WaitUntil(Instant::now() + remaining);
                return;
            }

            let app = app.read();
            let display_list = display_list.lock();
            redraw(&app, &display_list, &gl_context, &mut renderer);
            last_render = Instant::now();

            // Do not re-render unless there is a reason to
            //
            // This is why the window has 0 CPU usage when nothing is happening
            *control_flow = ControlFlow::Wait;
        },

        GlutinEvent::LoopDestroyed => {
            // Notify the server that it should shutdown, ignoring the error if the channel has
            // been dropped since that just means that the server task has ended already
            server_shutdown.blocking_send(()).unwrap_or(());
        },

        _ => {},
    });
}

fn redraw(
    app: &App,
    display_list: &DisplayList,
    gl_context: &WindowedContext<PossiblyCurrent>,
    renderer: &mut Renderer,
) {
    let draw_size = gl_context.window().inner_size();
    let drawing = app.drawing();
    let turtle_states = app.turtles().map(|(_, turtle)| &turtle.state);

    renderer.render(draw_size, display_list, drawing, turtle_states);
    gl_context.swap_buffers().expect("unable to swap the buffer (for double buffering)");
}

fn spawn_async_server(
    handle: &Handle,
    app: SharedApp,
    display_list: SharedDisplayList,
    event_loop: EventLoopNotifier,
    events_receiver: mpsc::UnboundedReceiver<Event>,
    establish_connection: impl Future<Output=Result<(ServerSender, ServerReceiver), ConnectionError>> + Send + 'static,
    server_shutdown_receiver: mpsc::Receiver<()>,
) {
    handle.spawn(async {
        let (conn_sender, conn_receiver) = establish_connection.await
            .expect("unable to establish turtle server connection");

        super::serve(
            conn_sender,
            conn_receiver,
            app,
            display_list,
            event_loop,
            events_receiver,
            server_shutdown_receiver,
        ).await;
    });
}
