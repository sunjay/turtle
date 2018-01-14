use std::sync::mpsc;
#[cfg(not(any(feature = "test", test)))]
use std::{env, thread, process};

use query::{Query, Response};
#[cfg(not(any(feature = "test", test)))]
use super::messenger::{self, Disconnected};

/// Manages the renderer process and all communication with it
#[cfg(not(any(feature = "test", test)))]
pub struct RendererProcess {
    process: process::Child,
    thread_handle: Option<thread::JoinHandle<()>>,
    /// Channel for receiving responses from the rendering process
    response_channel: mpsc::Receiver<Response>,
}

#[cfg(not(any(feature = "test", test)))]
impl RendererProcess {
    /// Spawn the renderer process and also a thread for communicating with that process
    pub fn new() -> Self {
        let current_exe = env::current_exe()
            .expect("Could not read path of the currently running executable")
            .into_os_string();
        let mut renderer_process = process::Command::new(current_exe)
            .env("RUN_TURTLE_CANVAS", "true")
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::inherit())
            .spawn()
            .expect("renderer process failed to start");

        let renderer_stdout = renderer_process.stdout.take()
            .expect("renderer process was not opened with stdout");
        let (response_tx, response_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            // Continously read responses from the renderer process
            // This is in its own thread because it uses blocking IO and we don't want to block
            // the main thread waiting for the renderer process
            messenger::read_forever(
                renderer_stdout,
                "bug: unable to read data from renderer process",
                "bug: failed to read response from renderer process",
                |resp| response_tx.send(resp).map_err(|_| Disconnected),
            );
        });

        Self {
            process: renderer_process,
            thread_handle: Some(handle),
            response_channel: response_rx,
        }
    }

    pub fn send_query(&mut self, query: Query) -> Option<Response> {
        messenger::send(
            match self.process.stdin {
                Some(ref mut stdin) => stdin,
                None => unreachable!("bug: renderer process was not opened with stdin"),
            },
            &query,
            "bug: unable to write final newline when sending query"
        ).unwrap_or_else(|_| {
            // Something went wrong while sending the query, check if the renderer process
            // panicked (exited with an error)
            match self.process.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        // The window/renderer process was closed normally
                        process::exit(0);
                    }
                    else {
                        // Something went wrong, likely the other thread panicked
                        process::exit(1);
                    }
                },
                Ok(None) => panic!("bug: failed to send query even though renderer process was still running"),
                Err(_) => panic!("bug: unable to check the exit status of the renderer process"),
            }
        });

        // Requests need responses
        if let Query::Request(_) = query {
            Some(self.wait_for_response())
        }
        else {
            None
        }
    }

    fn wait_for_response(&mut self) -> Response {
        self.response_channel.recv().unwrap_or_else(|_| {
            // The client thread has exited, that means that the renderer process has exited
            // and the window has closed
            self.exit_process()
        })
    }

    /// Exits the current process with the correct error code
    ///
    /// Panics if the thread handle has already been consumed
    #[inline]
    fn exit_process(&mut self) -> ! {
        let status = self.thread_handle.take().ok_or_else(|| {
            unreachable!("bug: the thread handle was used but the process did not end");
        }).and_then(|handle| {
            // First check if the other thread panicked before it quit
            handle.join().map_err(|_| ())
        }).and_then(|_| {
            // Then check if the renderer process ended normally
            self.process.wait()
                .map_err(|_| unreachable!("bug: renderer process never ran even though we exited"))
                .and_then(|status| if status.success() { Ok(()) } else { Err(()) })
        });

        match status {
            Ok(_) => process::exit(0),
            Err(_) => process::exit(1),
        }
    }
}

#[cfg(not(any(feature = "test", test)))]
impl Drop for RendererProcess {
    fn drop(&mut self) {
        // If the current thread is panicking, we want to abort right away
        // because otherwise there is code in the rendering thread that will call
        // process::exit(0) and then the exit code will be 0 instead of 1
        if thread::panicking() {
            process::exit(1);
        }

        // If this is just a normal ending of the main thread, we want to leave the renderer
        // running so that the user can see their drawing as long as they keep the window open
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap_or_else(|_| {
                // If this returns an error, the other thread panicked
                process::exit(1);
            });
        }

        // Now that the thread has completed, the process likely has as well (or is about to).
        // Check its exit status to see if we need to quit with an error
        match self.process.wait() {
            Ok(status) => if !status.success() {
                process::exit(1);
            },
            Err(_) => unreachable!("bug: renderer process never ran even though we exited"),
        }
    }
}

/// A special "renderer process" specifically for tests. Simulates the renderer process by
/// providing all of the same functionality and reusing internal parts of the server. No actual
/// process or additional threads are spawned.
#[cfg(any(feature = "test", test))]
pub struct RendererProcess {
    app: ::app::TurtleApp,
    events: (mpsc::Sender<::Event>, mpsc::Receiver<::Event>),
    drawing: (mpsc::Sender<::query::DrawingCommand>, mpsc::Receiver<::query::DrawingCommand>),
}

#[cfg(any(feature = "test", test))]
impl RendererProcess {
    pub fn new() -> Self {
        Self {
            app: ::app::TurtleApp::new(),
            events: mpsc::channel(),
            drawing: mpsc::channel(),
        }
    }

    pub fn send_query(&mut self, query: Query) -> Option<Response> {
        super::server::handle_query_for_test_use_only(query, &mut self.app, &self.events.1, &self.drawing.0)
            .expect("test bug: a query failed to be successful")
    }
}
