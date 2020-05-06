use std::io;
use std::env;
use std::process::{Stdio, exit};

use tokio::{
    runtime::Handle,
    io::AsyncWriteExt,
    task::JoinHandle,
    process::{Command, ChildStdin},
};

/// The environment variable that is set to indicate that the current process is a server process
pub const RENDERER_PROCESS_ENV_VAR: &str = "RUN_TURTLE_CANVAS";

#[derive(Debug)]
pub struct RendererServerProcess {
    task_handle: Option<JoinHandle<()>>,
    /// A handle to the stdin of the child process
    child_stdin: ChildStdin,
}

impl RendererServerProcess {
    /// Spawn a new process for the renderer
    pub async fn spawn() -> Self {
        let current_exe = env::current_exe()
            .expect("Could not read path of the currently running executable");

        // The new process is the same executable as this process but with a special environment
        // variable passed in
        let mut child = Command::new(current_exe)
            .env(RENDERER_PROCESS_ENV_VAR, "true")
            // Pipe input so we can communicate with the spawned process
            //
            // stdout/stderr will be inherited from the current process
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to start separate process for renderer");

        let child_stdin = child.stdin.take()
            .expect("renderer process was not spawned with a handle to stdin");

        // Ensure the child process is spawned in the runtime so it can
        // make progress on its own while we send it input.
        let task_handle = Some(tokio::spawn(async {
            // We want to await in a separate task because otherwise the current task would not
            // make any progress until the child process is complete
            let status = child.await
                .expect("child process encountered an error");

            if status.success() {
                // The window/renderer process was closed normally
                exit(0);
            } else {
                // Something went wrong, likely the other thread panicked
                exit(1);
            }
        }));

        Self {task_handle, child_stdin}
    }

    /// Writes the given bytes followed by a newline b'\n' to the stdin of the process
    ///
    /// Unlike `std::io::Write::write`, this returns an error in the case all of the bytes could
    /// not be written for some reason.
    pub async fn writeln<S: AsRef<[u8]>>(&mut self, data: S) -> io::Result<()> {
        let data = data.as_ref();
        self.child_stdin.write_all(data).await?;
        self.child_stdin.write_all(&[b'\n']).await?;

        Ok(())
    }
}

impl Drop for RendererServerProcess {
    fn drop(&mut self) {
        use std::thread;

        // If the current thread is panicking, we want to abort right away
        // because otherwise there is code in the rendering thread that will call
        // process::exit(0) and then the exit code will be 0 instead of 1
        if thread::panicking() {
            exit(1);
        }

        // If this is just a normal ending of the main thread, we want to leave the renderer
        // running so that the user can see their drawing as long as they keep the window open
        let runtime = Handle::current();
        // This unwrap is safe because no struct gets dropped twice
        let task_handle = self.task_handle.take().unwrap();
        runtime.block_on(task_handle).unwrap_or_else(|_| {
            // If this returns an error, the other thread panicked
            exit(1);
        });
    }
}
