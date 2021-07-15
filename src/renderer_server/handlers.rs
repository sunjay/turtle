mod animation;
mod clear;
mod create_turtle;
mod debug;
mod destroy_drawing;
mod drawing_prop;
mod export_drawings;
mod fill;
mod poll_event;
mod turtle_prop;

pub(crate) use animation::*;
pub(crate) use clear::*;
pub(crate) use create_turtle::*;
pub(crate) use debug::*;
pub(crate) use destroy_drawing::*;
pub(crate) use drawing_prop::*;
pub(crate) use export_drawings::*;
pub(crate) use fill::*;
pub(crate) use poll_event::*;
pub(crate) use turtle_prop::*;

use thiserror::Error;

use super::event_loop_notifier::EventLoopClosed;

#[derive(Debug, Error)]
#[error(transparent)]
pub(crate) enum HandlerError {
    /// Unable to send response to IPC client
    IpcChannelError(#[from] ipc_channel::Error),

    EventLoopClosed(#[from] EventLoopClosed),
}
