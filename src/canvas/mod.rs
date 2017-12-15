extern crate graphics;

use std::mem;
use std::ffi::CString;
use std::os::raw::c_char;
use std::heap::{Alloc, Heap, Layout};
use std::slice;

use graphics::{Graphics};
use query::{Query, Request, Response, StateUpdate};
use renderer::Renderer;
use runtime::Runtime;
use app::TurtleApp;
use clock;

pub mod rgba_buffer_graphics;
use self::rgba_buffer_graphics::RgbaBufferGraphics;

static mut TEMP_HACK_BUFFER: &'static mut [u8] = &mut [0; 500 * 500 * 4];

/// A runtime for hosting turtle logic in a web page via WebAssembly and the `<canvas>` element.
pub struct CanvasRuntime<'a> {
    renderer: Renderer,
    context: graphics::Context,
    app: TurtleApp,
    graphics: rgba_buffer_graphics::RgbaBufferGraphics<'a>
}

impl<'a> Runtime for CanvasRuntime<'a> {
    type Clock = WebClock;

    fn initialize() {
        // no op
    }

    fn send_query(&mut self, query: Query) -> Option<Response> {
        match query {
            Query::Request(r) => return match r {
                Request::TurtleState => Some(Response::TurtleState(self.app.turtle().clone())),
                Request::DrawingState => Some(Response::DrawingState(self.app.drawing().clone())),
                Request::Event => {
                    // TODO expose events from browser
                    None
                }
            },
            Query::Update(u) => match u {
                StateUpdate::TurtleState(s) => *self.app.turtle_mut() = s,
                StateUpdate::DrawingState(d) => *self.app.drawing_mut() = d,
                StateUpdate::TemporaryPath(p) => self.app.set_temporary_path(p),
            }
            Query::Drawing(d) => {
                self.renderer.handle_drawing_command(d)
            }
        }

        // TODO when should we draw?

        let state = self.app.read_only();
        let drawing = state.drawing().clone();
        let temporary_path = state.temporary_path().clone();
        let turtle = state.turtle().clone();

        let view = self.context.get_view_size();
        let width = view[0] as f64;
        let height = view[1] as f64;

        let center = [width * 0.5, height * 0.5];

        self.renderer.render(self.context, &mut self.graphics, center, &drawing, &temporary_path, &turtle);

        None
    }

    fn debug_log(s: &str) {
        let mut v = Vec::new();
        v.extend(s.as_bytes().iter());
        v.push(0);

        unsafe {
            web_debug_log(v.as_ptr() as *const u8)
        }

        // ensure v lives until here so the pointer is valid when read above
        mem::drop(v);
    }
}

impl<'a> Default for CanvasRuntime<'a> {
    fn default() -> Self {
        unsafe {
            CanvasRuntime {
                renderer: Renderer::new(),
                context: graphics::Context::new(),
                app: TurtleApp::new(),
                // TODO initialization
                graphics: rgba_buffer_graphics::RgbaBufferGraphics::new(500, 500, &mut TEMP_HACK_BUFFER)
            }
        }
    }
}

extern "C" {
    fn web_current_timestamp() -> f64;
    /// Log the UTF8 null-terminated string at this pointer.
    /// The pointer will be freed on this end once the call is complete.
    fn web_debug_log(c_str_utf8: *const u8);
}

pub struct WebTimestamp {
    ms: f64
}

impl clock::Timestamp for WebTimestamp {
    fn elapsed(&self) -> f64 {
        <WebClock as clock::Clock>::now().ms - self.ms
    }
}

/// Clock backed by the web Performance API
pub struct WebClock;

impl clock::Clock for WebClock {
    type Timestamp = WebTimestamp;

    fn now() -> Self::Timestamp {
        let ms = unsafe {
            web_current_timestamp()
        };

        WebTimestamp {
            ms
        }
    }
}

// functions used from js

#[no_mangle]
pub fn web_turtle_start(pointer: *mut u8, max_width: usize, max_height: usize) {

    // pixels are stored in RGBA, so each pixel is 4 bytes
    let byte_size = max_width * max_height * 4;
    let mut sl = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    let mut g = RgbaBufferGraphics::new(max_width, max_height, &mut sl);

    g.clear_color([0.7, 0.0, 0.7, 1.0]);

    // circle example
    let mut turtle = ::turtle::Turtle::new();

    use runtime::Runtime;
    for _ in 0..10 {
        CanvasRuntime::debug_log("another degree");
        // Move forward three steps
        turtle.forward(3.0);
        // Rotate to the right (clockwise) by 1 degree
        turtle.right(1.0);
    }
}


#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
        Heap.alloc(layout).unwrap()
    }
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
        Heap.dealloc(ptr, layout);
    }
}
