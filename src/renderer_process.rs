use std::env;
use std::thread;
use std::process::{self, Stdio};
use std::sync::mpsc;

use client;
use query::{Query, Response};

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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("renderer process failed to start");

        let renderer_stdout = renderer_process.stdout.take()
            .expect("renderer process was not opened with stdout");
        let (response_tx, response_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            client::run(renderer_stdout, response_tx);
        });

        Self {
            process: renderer_process,
            thread_handle: Some(handle),
            response_channel: response_rx,
        }
    }

    /// Sends a query and automatically decides whether or not to wait for a response.
    ///
    /// If a query does not require a response, this function will return immediately after
    /// sending the query
    pub fn send_query(&mut self, query: Query) -> Option<Response> {
        client::send_query(match self.process.stdin {
            Some(ref mut stdin) => stdin,
            None => unreachable!("bug: renderer process was not opened with stdin"),
        }, &query).unwrap_or_else(|_| {
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
        match self.response_channel.recv() {
            Ok(response) => response,
            // The client thread has exited, that means that the renderer process has exited
            // and the window has closed
            Err(_) => self.exit_process(), // Quit
        }
    }

    /// Exits the current process with the correct error code
    ///
    /// Panics if the thread handle has already been consumed
    #[inline]
    fn exit_process(&mut self) -> ! {
        if let Some(handle) = self.thread_handle.take() {
            // First check if the other thread panicked before it quit
            match handle.join() {
                Ok(_) => match self.process.try_wait() {
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
                    Ok(None) => match self.process.wait() {
                        Ok(status) => {
                            if status.success() {
                                process::exit(0);
                            }
                            else {
                                process::exit(1);
                            }
                        },
                        Err(_) => unreachable!("bug: renderer process never ran even though we exited"),
                    },
                    Err(_) => panic!("bug: unable to check the exit status of the renderer process after client thread quit"),
                },
                // If this returns an error, the other thread panicked
                Err(_) => process::exit(1),
            }
        }
        else {
            unreachable!("bug: the thread handle was used but the process did not end");
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
