use std::io;
use std::env;
use std::process::{self, Stdio, ExitStatus};

use tokio::{
    runtime,
    io::AsyncWriteExt,
    process::{Command, ChildStdin},
};
use futures_util::future::{FutureExt, RemoteHandle};

/// The environment variable that is set to indicate that the current process is a server process
pub const RENDERER_PROCESS_ENV_VAR: &str = "RUN_TURTLE_CANVAS";

#[derive(Debug)]
pub struct RendererServerProcess {
    /// A handle to the runtime that the process was spawned in. This is needed because a handle
    /// to the runtime can only be created when a "runtime context". Since Drop may not always run
    /// from async code, we need this to ensure we can wait on the subprocess in `task_handle`.
    /// NOTE: This creates an implicit invariant that this struct must be dropped before the
    /// runtime that it was created in is dropped. This is not an issue in normal code and will at
    /// worst cause a panic!().
    runtime_handle: runtime::Handle,
    /// A handle to the running task. This can be waited on to find out if the process exited
    /// successfully. A remote handle will also drop the future it is associated with when it is
    /// dropped. (unlike a `JoinHandle` which will detach instead.) This is important to make sure
    /// the window closes when the thread holding this struct panics.
    task_handle: Option<RemoteHandle<io::Result<ExitStatus>>>,
    /// A handle to the stdin of the child process
    child_stdin: ChildStdin,
}

impl RendererServerProcess {
    /// Spawn a new process for the renderer
    pub async fn spawn() -> io::Result<Self> {
        let current_exe = env::current_exe()?;

        // The new process is the same executable as this process but with a special environment
        // variable passed in
        let mut child = Command::new(current_exe)
            .env(RENDERER_PROCESS_ENV_VAR, "true")
            // Pipe input so we can communicate with the spawned process
            //
            // stdout/stderr will be inherited from the current process
            .stdin(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let child_stdin = child.stdin.take()
            .expect("renderer process was not spawned with a handle to stdin");

        // Spawn a separate task for the child process so this task can continue to make progress
        // while that runs. The remote handle will drop that future when it is dropped.
        let (child, child_handle) = child.remote_handle();
        tokio::spawn(child);
        let task_handle = Some(child_handle);

        // Keep a handle to the current runtime
        let runtime_handle = runtime::Handle::current();

        Ok(Self {runtime_handle, task_handle, child_stdin})
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

        // If the current thread is panicking, we don't want to wait for the window to be closed
        // since the user likely has an issue to fix in their program.
        if thread::panicking() {
            // The process is configured with `kill_on_drop` so this should close the window
            return;
        }

        // If this is just a normal ending of the main thread, we want to leave the renderer
        // running so that the user can see their drawing as long as they keep the window open

        // This unwrap is safe because no struct gets dropped twice
        let task_handle = self.task_handle.take().unwrap();

        // Wait for the child process to finish
        match self.runtime_handle.block_on(task_handle) {
            Ok(proc_status) => if !proc_status.success() {
                // Propagate error code from child process or exit with status code 1
                process::exit(proc_status.code().unwrap_or(1));
            },
            Err(err) => {
                panic!("error while running child process: {}", err);
            },
        }
    }
}
