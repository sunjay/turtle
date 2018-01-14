extern crate graphics;

use std::mem;
use std::heap::{Alloc, Heap, Layout};

use query::{Query, Request, Response, StateUpdate};
use renderer::Renderer;
use runtime::Runtime;
use app::TurtleApp;
use ::{clock, Point};

pub mod rgba_buffer_graphics;

/// A runtime for hosting turtle logic in a web page via WebAssembly and the `<canvas>` element.
pub struct CanvasRuntime {
    renderer: Renderer,
    context: graphics::Context,
    app: TurtleApp,
    graphics: rgba_buffer_graphics::RgbaBufferGraphics
}

impl CanvasRuntime {
    pub fn new(width: usize, height: usize, pixel_buffer: *mut u8) -> Self {
        CanvasRuntime {
            renderer: Renderer::new(),
            context: graphics::Context::new(),
            app: TurtleApp::new(),
            graphics: rgba_buffer_graphics::RgbaBufferGraphics::new(width, height, pixel_buffer),
        }
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

impl Runtime for CanvasRuntime {
    type Clock = WebClock;
    type Rng = WebRng;

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

        let center = Point {x: width * 0.5, y: height * 0.5};

        self.renderer.render(self.context, &mut self.graphics, center, &drawing, &temporary_path, &turtle);

        unsafe {
            web_update_canvas();
        }

        None
    }

    fn rng() -> Self::Rng {
        WebRng{}
    }
}

extern "C" {
    fn web_current_timestamp() -> f64;
    /// Log the UTF8 null-terminated string at this pointer.
    /// The pointer will be freed on this end once the call is complete.
    fn web_debug_log(c_str_utf8: *const u8);

    /// A non-cryptographically secure "js number" in [0, u32 max].
    fn web_prng() -> u32;

    /// Copy pixels from the buffer to the canvas element
    fn web_update_canvas();
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

// Rng backed by JS's random
pub struct WebRng;

impl ::rand::Rng for WebRng {
    fn next_u32(&mut self) -> u32 {
        unsafe {
            web_prng()
        }
    }
}

// functions used from js

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
