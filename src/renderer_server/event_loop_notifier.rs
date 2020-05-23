use glutin::{
    dpi::LogicalSize,
    event_loop::{self, EventLoopProxy},
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("event loop closed while messages were still being sent to it")]
pub struct EventLoopClosed;

impl<T> From<event_loop::EventLoopClosed<T>> for EventLoopClosed {
    fn from(_: event_loop::EventLoopClosed<T>) -> Self {
        EventLoopClosed
    }
}

/// A custom event used to perform actions within the glutin event loop on the main thread
#[derive(Debug, Clone, PartialEq)]
pub enum MainThreadAction {
    /// Redraw the window
    Redraw,
    /// Update the window title
    SetTitle(String),
    /// Update the window size (in logical coordinates)
    SetSize(LogicalSize<u32>),
    /// Change the maximized state of the window
    SetIsMaximized(bool),
    /// Change the fullscreen state of the window
    SetIsFullscreen(bool),
}

/// Notifies the main loop when actions need to take place
#[derive(Debug, Clone)]
pub struct EventLoopNotifier {
    event_loop: EventLoopProxy<MainThreadAction>,
}

impl EventLoopNotifier {
    pub fn new(event_loop: EventLoopProxy<MainThreadAction>) -> Self {
        Self {event_loop}
    }

    pub fn request_redraw(&self) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::Redraw)
    }

    pub fn set_title(&self, title: String) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetTitle(title))
    }

    pub fn set_size<S: Into<LogicalSize<u32>>>(&self, size: S) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetSize(size.into()))
    }

    pub fn set_is_maximized(&self, is_maximized: bool) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetIsMaximized(is_maximized))
    }

    pub fn set_is_fullscreen(&self, is_fullscreen: bool) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetIsFullscreen(is_fullscreen))
    }

    fn send_action(&self, action: MainThreadAction) -> Result<(), EventLoopClosed> {
        Ok(self.event_loop.send_event(action)?)
    }
}
