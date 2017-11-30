// During tests, we disable the renderer and that causes a bunch of warnings that we just want
// to get rid of.
// See Cargo.toml for an explanation of this attribute
#![cfg_attr(any(feature = "test", test), allow(dead_code, unused_variables, unused_imports))]

use std::env;
use std::thread;
use std::process;
use std::sync::mpsc;

use app::TurtleApp;
use renderer::Renderer;
use server;

/// Start the turtle canvas process.
///
/// You must call this function at the beginning of `main()` if you do not create the
/// turtle immediately using [`Turtle::new()`].
///
/// It's a good idea to call this function before any other code runs in `main()`. Programs that
/// parse command line arguments or look at environment variables may **fail** to start if this
/// function is not called right at the beginning of the program. Programs that perform any
/// expensive computations may experience delayed start up problems unless they call this
/// function first.
///
/// The [`Turtle::new()`] method will call this function for you so that you don't need to worry
/// about this unless you are doing something before that.
///
/// # Example
/// ```rust,no_run
/// # #![allow(unused_variables, unused_mut)]
/// extern crate turtle;
/// use turtle::{self, Turtle};
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
    // code runs after canvas::run(). This allows us to ship one executable that appears to
    // have two separate processes.
    // We run the renderer loop and then immediately exit.
    if env::var("RUN_TURTLE_CANVAS").unwrap_or_else(|_| "".to_owned()) == "true" {
        // This code MUST be run on the main thread.

        // Run the canvas process
        main();
        unreachable!("bug: renderer loop did not exit after finishing");
    }
}

pub fn main() {
    let app = TurtleApp::new();
    let read_only = app.read_only();
    let (drawing_tx, drawing_rx) = mpsc::channel();
    let (events_tx, events_rx) = mpsc::channel();

    let (running_tx, running_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        server::run(app, drawing_tx, events_rx, running_tx);
    });

    // Renderer MUST run on the main thread or else it will panic on MacOS
    #[cfg(not(any(feature = "test", test)))]
    Renderer::new().run(drawing_rx, events_tx, read_only);

    // Quit immediately when the window is closed

    // Check if an error has occurred on the thread
    match running_rx.try_recv() {
        Ok(_) => unreachable!("bug: running channel should always be empty"),
        // The thread was still running, exit normally
        Err(mpsc::TryRecvError::Empty) => process::exit(0),
        Err(mpsc::TryRecvError::Disconnected) => match handle.join() {
            Ok(_) => process::exit(0),
            // The other thread must have panicked
            Err(_) => process::exit(1),
        },
    }
}
