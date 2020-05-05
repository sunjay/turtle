use std::env;

use super::main::main;

use crate::renderer_client::RENDERER_PROCESS_ENV_VAR;

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
    // If this environment variable is present, this process is taken over so that no other
    // code runs after main(). This allows us to ship one executable that appears to
    // have two separate processes.
    //
    // This implementation detail is why we request that users run start() at the beginning of
    // their programs. When we spawn the same executable, we don't pass along any environment,
    // input or command line arguments. That means that the user *needs* to run start() first or
    // else their program won't be able to run at all. This is a tradeoff of this design decision.
    if env::var(RENDERER_PROCESS_ENV_VAR).ok().as_deref() == Some("true") {
        // This code MUST be run on the main thread.

        // Run the renderer process
        main();
        unreachable!("bug: renderer loop did not exit after finishing");
    }
}
