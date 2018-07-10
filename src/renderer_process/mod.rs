//! This module swaps between renderer backends based on the current build configuration.
//!
//! * desktop - separate renderer process
//! * test - no-op query handling
//! * wasm - JavaScript FFI
//!
//! You can think of this module as being the dispatcher for the different "rendering backends".

#[cfg(all(feature = "desktop", not(any(feature = "test", test))))]
mod desktop;
#[cfg(all(feature = "desktop", not(any(feature = "test", test))))]
pub use self::desktop::*;

#[cfg(all(target_arch = "wasm32", not(any(feature = "test", test))))]
mod wasm;
#[cfg(all(target_arch = "wasm32", not(any(feature = "test", test))))]
pub use self::wasm::*;

#[cfg(any(feature = "test", test))]
mod test;
#[cfg(any(feature = "test", test))]
pub use self::test::*;
