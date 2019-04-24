#[cfg(not(target_arch = "wasm32"))]
compile_error!("This module should only be included when compiling to wasm");

use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::mem;
use std::os::raw::{c_char, c_void};

use crate::query::{Query, Response};

// Functions preovided by JavaScript, to be called by the WebAssembly generated from Rust
extern "C" {
    fn send_query(query: *const c_char) -> *const c_char;
    fn _log(message: *const c_char);
}

// In order to work with the memory in WASM, we expose allocation and deallocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *const c_void {
    let buf = Vec::with_capacity(size);
    let ptr = buf.as_ptr();
    // Forget the pointer so that Rust doesn't free the memory we want to give JavaScript
    // access to. (Leaking memory is **safe**, so unsafe { ... } is not necessary.)
    mem::forget(buf);
    return ptr as *const c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        // Rust will drop this vector and free the memory
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

/// Specifically for deallocating NULL-terminated strings without knowing their length in advance
///
/// See `CString::into_raw` for more information:
/// https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_raw
#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

/// Prints a value out to the JavaScript console (for debugging purposes)
pub(crate) fn _debug<T: Debug>(value: T) {
    let raw_str = CString::new(format!("{:?}", value)).unwrap().into_raw();
    unsafe { _log(raw_str) };
}

/// A special "renderer process" specifically for communicating through the web assembly boundary
/// to the JavaScript that is running this program.
pub struct RendererProcess {}

impl RendererProcess {
    pub fn new() -> Self {
        Self {}
    }

    pub fn send_query(&mut self, query: Query) -> Option<Response> {
        let query_str = serde_json::to_string(&query).unwrap();
        let c_str = CString::new(query_str).unwrap();
        // Once the string is passed into JavaScript, JavaScript is now considered the owner of
        // that string.
        let raw_str = c_str.into_raw();
        let response_cstr = unsafe { CStr::from_ptr(send_query(raw_str)) };
        // Requests need responses
        if let Query::Request(_) = query {
            let response_str = response_cstr.to_str().expect("String provided by JavaScript was not valid UTF-8");
            let response = serde_json::from_str(response_str).expect("String provided by JavaScript was not valid JSON");
            Some(response)
        } else {
            None
        }
    }
}
