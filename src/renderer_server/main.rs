use std::thread;
use std::time::{Instant, Duration};
use std::sync::Arc;

use glutin::{
    GlProfile,
    GlRequest,
    ContextBuilder,
    WindowedContext,
    PossiblyCurrent,
    dpi::LogicalSize,
    window::{WindowBuilder, Fullscreen},
    event::{
        Event as GlutinEvent,
        StartCause,
        WindowEvent,
        DeviceEvent,
        KeyboardInput,
        VirtualKeyCode,
        ElementState,
    },
    event_loop::{ControlFlow, EventLoop},
};
use tokio::{
    sync::{mpsc, Mutex},
    runtime::{Runtime, Handle},
};

use crate::Event;
use crate::ipc_protocol::ServerConnection;

use super::{
    app::App,
    renderer::{
        Renderer,
        display_list::DisplayList,
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

/// Run the renderer process in the current thread
///
/// This function must run in the main thread ONLY
pub fn main() {
    assert_main_thread();

    // The runtime for driving async code
    let runtime = Runtime::new()
        .expect("unable to spawn tokio runtime to run turtle server process");

    // The state of the drawing and the state/drawings associated with each turtle
    let app = Arc::new(App::default());
    // All of the drawing primitives in the order in which they wil be drawn
    //
    // This is managed separately from the rest of the app state because the display list is shared
    // among pretty much everything and so critical sections containing the display list need to be
    // as short as possible.
    let display_list = Arc::new(Mutex::new(DisplayList::default()));

    let event_loop = EventLoop::with_user_event();
    // Create the proxy that will be given to the thread managing IPC
    let event_loop_proxy = event_loop.create_proxy();
    let event_loop_notifier = Arc::new(EventLoopNotifier::new(event_loop_proxy));

    // A channel for transferring events
    let (events_sender, events_receiver) = mpsc::unbounded_channel();
    // Put the events_receiver in an Option so we can call `take()` in the event loop. Required
    // because borrow checker cannot verify that `Init` event is only fired once.
    let mut events_receiver = Some(events_receiver);

    let window_builder = {
        let drawing = runtime.handle().block_on(app.drawing_mut());
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
    event_loop.run(move |event, _, control_flow| match event {
        GlutinEvent::NewEvents(StartCause::Init) => {
            // Spawn the actual server thread(s) that will handle incoming IPC messages and
            // asynchronous update the shared state
            //
            // Note that putting this code here instead of before the event loop causes the
            // `Turtle::new()`, etc. methods not to return before the window opens. Those methods
            // can't return because the connection handshake cannot complete before the thread used
            // for IPC is spawned.
            let handle = runtime.handle().clone();
            spawn_async_server(
                handle,
                app.clone(),
                display_list.clone(),
                event_loop_notifier.clone(),
                events_receiver.take().expect("bug: init event should only occur once"),
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

        GlutinEvent::WindowEvent {event, ..} => {
            //TODO: Check if event modifies state and then redraw if necessary
            match event {
                _ => {}
            }

            //TODO: There is no guarantee that sending this event here will actually allow a client
            // to receive it. After all, if the window closes and this process exits, there will be
            // no way to handle subsequent `NextEvent` requests.
            let scale_factor = renderer.scale_factor();
            if let Some(event) = Event::from_window_event(event, scale_factor) {
                events_sender.send(event)
                    .expect("bug: server IPC thread should stay alive as long as server main thread");
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
            //TODO: No idea if this next line is necessary or not
            gl_context.window().request_redraw();
        },

        GlutinEvent::UserEvent(MainThreadAction::SetIsMaximized(is_maximized)) => {
            gl_context.window().set_maximized(is_maximized);
            //TODO: No idea if this next line is necessary or not
            gl_context.window().request_redraw();
        },

        GlutinEvent::UserEvent(MainThreadAction::SetIsFullscreen(is_fullscreen)) => {
            gl_context.window().set_fullscreen(if is_fullscreen {
                Some(Fullscreen::Borderless(gl_context.window().current_monitor()))
            } else { None });

            //TODO: No idea if this next line is necessary or not
            gl_context.window().request_redraw();
        },

        GlutinEvent::RedrawRequested(_) => {
            // Check if we just rendered
            let last_render_delay = last_render.elapsed();
            if last_render_delay < min_render_delay {
                let remaining = min_render_delay - last_render_delay;
                *control_flow = ControlFlow::WaitUntil(Instant::now() + remaining);
                return;
            }

            let handle = runtime.handle();
            handle.block_on(redraw(&app, &display_list, &gl_context, &mut renderer));
            *control_flow = ControlFlow::Wait;
            last_render = Instant::now();
        },

        _ => {},
    });
}

async fn redraw(
    app: &App,
    display_list: &Mutex<DisplayList>,
    gl_context: &WindowedContext<PossiblyCurrent>,
    renderer: &mut Renderer,
) {
    let drawing = {
        // Hold the drawing lock for as little time as possible
        let drawing = app.drawing_mut().await;
        drawing.clone()
    };

    // Locking the turtles before the display list to be consistent with all of the request
    // handlers. Inconsistent lock ordering can cause deadlock.
    let mut turtles = Vec::new();
    for id in app.turtle_ids().await {
        turtles.push(app.turtle(id).await);
    }
    // Very important to have all the data locked before rendering. Do not want renderer to have
    // to figure out how to lock.
    let mut locked_turtles = Vec::with_capacity(turtles.len());
    for turtle in &turtles {
        locked_turtles.push(turtle.lock().await);
    }
    // Renderer only needs (read-only) access to TurtleState
    // Doing this also decouples renderer code from runtime by not having to
    // know about the `MutexGuard` type in tokio
    let turtle_states = locked_turtles.iter().map(|t| &t.state);

    let display_list = display_list.lock().await;

    let draw_size = gl_context.window().inner_size();
    renderer.render(draw_size, &display_list, &drawing, turtle_states);
    gl_context.swap_buffers().expect("unable to swap the buffer (for double buffering)");
}

fn assert_main_thread() {
    // This check isn't foolproof. Someone can always create a thread named "main".
    if thread::current().name().unwrap_or("") != "main" {
        // In order to maintain compatibility with MacOS, we need to make sure that windows are
        // only created on the main thread. We do this check on all platforms so that no one
        // can accidentally make a change that creates the window off of the main thread.
        //
        // It's easy for a user to accidentally cause this panic if they call `Turtle::new()` in a
        // new thread. This message is meant to point them to the solution: `turtle::start()`
        panic!("Windows can only be created on the main thread. \
                Make sure you have called `turtle::start()` at the beginning of your program. \
                See: <https://docs.rs/turtle/*/turtle/fn.start.html>");
    }
}

fn spawn_async_server(
    handle: Handle,
    app: Arc<App>,
    display_list: Arc<Mutex<DisplayList>>,
    event_loop: Arc<EventLoopNotifier>,
    events_receiver: mpsc::UnboundedReceiver<Event>,
) {
    // Spawn root task
    handle.spawn(async {
        let conn = ServerConnection::connect_stdin().await
            .expect("unable to establish turtle server connection");
        super::serve(conn, app, display_list, event_loop, events_receiver).await;
    });
}
