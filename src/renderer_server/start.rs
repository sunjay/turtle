#[cfg(not(any(feature = "test", test)))]
use std::sync::atomic::{AtomicBool, Ordering};

use super::backend::RendererServer;

/// `start()` must be called once from the main thread, but it can be called after that any number
/// of times. This flag helps ensure that the main thread check only executes the first time.
#[cfg(not(any(feature = "test", test)))]
static START_RAN_ONCE: AtomicBool = AtomicBool::new(false);

/// Start the turtle window in advance
///
/// If you do not create a turtle immediately at the beginning of `main()` with [`Turtle::new()`],
/// you must call this function at the start of `main()` to avoid any problems.
///
/// Since the majority of code created using this crate does little or no work before calling
/// [`Turtle::new()`], this usually isn't a problem. Programs that parse command line arguments,
/// read input, or check environment variables may **fail** to start if this function is not called
/// right at the beginning of the program. Programs that perform any expensive computations may
/// experience delayed start up problems unless they call this function first.
///
/// The [`Turtle::new()`] method will call this function for you so that you don't need to worry
/// about this unless you are doing something before that.
///
/// # Example
/// ```rust,no_run
/// # #![allow(unused_variables, unused_mut)]
/// use turtle::Turtle;
///
/// fn main() {
///     // Initializes the turtle renderer first so that there is less delay when a Turtle
///     // is created and so that there are no conflicts with command line arguments or
///     // environment variables.
///     // Not required if Turtle::new() is already at the top of main.
///     turtle::start();
///
///     // Do all kinds of expensive work here...
///     // Feel free to check environment variables, command line arguments, etc.
///
///     // Create the turtle when you are ready
///     // Turtle::new() will also call start(), but calling it twice doesn't matter
///     let mut turtle = Turtle::new();
///     // Do things with the turtle...
/// }
/// ```
///
/// [`Turtle::new()`]: struct.Turtle.html#method.new
pub fn start() {
    // This check is performed on all platforms to help avoid compatibility hazards that may
    // accidentally make it harder to run a turtle program on a different platform. The check is
    // not foolproof and there is no way to verify that start() is called at the beginning of
    // main() in all cases. This is just to help in the cases where we can detect something.
    #[cfg(not(any(feature = "test", test)))]
    if !START_RAN_ONCE.swap(true, Ordering::SeqCst) {
        assert_main_thread();
    }

    RendererServer::start();
}

#[cfg(not(any(feature = "test", test)))]
fn assert_main_thread() {
    // This check isn't foolproof. Someone can always create a thread named "main".
    if std::thread::current().name().unwrap_or("") != "main" {
        // In order to maintain compatibility with MacOS, we need to make sure that windows are
        // only created on the main thread. We do this check on all platforms so that no one
        // can accidentally make a change that creates the window off of the main thread.
        //
        // It's easy for a user to accidentally cause this panic if they call `Turtle::new()` in a
        // new thread. This message is meant to point them to the solution: `turtle::start()`
        panic!("Windows can only be created on the main thread. Make sure you have called \
                `turtle::start()` at the beginning of your program's main() function. \
                See: <https://docs.rs/turtle/*/turtle/fn.start.html>");
    }
}
