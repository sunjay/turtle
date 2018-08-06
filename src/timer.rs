#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(any(feature = "test", test), allow(dead_code))]
pub struct Timer {
    timer: Instant,
}

#[cfg(not(target_arch = "wasm32"))]
impl Timer {
    pub fn start() -> Self {
        Self { timer: Instant::now() }
    }

    #[cfg_attr(any(feature = "test", test), allow(dead_code))]
    pub fn elapsed_millis(&self) -> u64 {
        Self::as_millis(self.timer.elapsed())
    }

    /// Converts the given Duration into its value in milliseconds
    ///
    /// This used to be part of the API but it is easy enough to compute
    /// from the current one as well.
    fn as_millis(duration: Duration) -> u64 {
        //FIXME: Remove this function and just use `timer.elapsed().as_millis()` when
        // this issue is closed: https://github.com/rust-lang/rust/issues/50202
        duration.as_secs() * 1000 + u64::from(duration.subsec_nanos() / 1_000_000)
    }
}

// Functions preovided by JavaScript, to be called by the WebAssembly generated from Rust
#[cfg(all(target_arch = "wasm32", not(any(feature = "test", test))))]
extern "C" {
    // WASM doesn't support passing i64 between JavaScript, so we can only use u32 here, not u64
    fn now() -> u32;
}

#[cfg(all(target_arch = "wasm32", not(any(feature = "test", test))))]
pub struct Timer {
    start: u64,
}

#[cfg(all(target_arch = "wasm32", not(any(feature = "test", test))))]
impl Timer {
    pub fn start() -> Self {
        Self { start: Self::now() }
    }

    pub fn elapsed_millis(&self) -> u64 {
        Self::now() - self.start
    }

    fn now() -> u64 {
        (unsafe { now() }) as u64
    }
}
