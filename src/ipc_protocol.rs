//! Handles the IPC (Inter-process Communication) protocol used by the renderer client and server.
//!
//! This is the common "language" between client/server. It defines both the protocol for
//! connection and for send messages back and forth.

mod messages;

pub use messages::*;

use ipc_channel::ipc::{IpcOneShotServer, IpcSender, IpcReceiver};

use crate::renderer_server::RendererServerProcess;

/// The environment variable that is set to indicate that the current process is a server process
pub const RENDERER_PROCESS_ENV_VAR: &str = "RUN_TURTLE_CANVAS";

/// Represents one end of an IPC connection
///
/// The connection is parameterized by the type of message that will be sent from this end and
/// received from the other end.
pub struct Connection<Send, Recv> {
    sender: IpcSender<Send>,
    receiver: IpcReceiver<Recv>,
}

impl<Send, Recv> Connection<Send, Recv> {
    pub fn new(process: &mut RendererServerProcess) -> Self {
        todo!()
    }
}
