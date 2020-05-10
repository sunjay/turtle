mod create_turtle;
mod export_drawings;
mod drawing_prop;
mod turtle_prop;
mod animation;
mod fill;
mod clear;

pub(crate) use create_turtle::*;
pub(crate) use export_drawings::*;
pub(crate) use drawing_prop::*;
pub(crate) use turtle_prop::*;
pub(crate) use animation::*;
pub(crate) use fill::*;
pub(crate) use clear::*;

use thiserror::Error;
use glutin::event_loop::EventLoopClosed;

#[derive(Debug, Error)]
#[error(transparent)]
pub(crate) enum HandlerError {
    /// Unable to send response to IPC client
    IpcChannelError(#[from] ipc_channel::Error),

    #[error("event loop closed while messages were still being sent to it")]
    EventLoopClosed,
}

impl<T> From<EventLoopClosed<T>> for HandlerError {
    fn from(_: EventLoopClosed<T>) -> Self {
        HandlerError::EventLoopClosed
    }
}
