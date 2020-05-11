use tokio::sync::Mutex;
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
#[derive(Debug)]
pub struct EventLoopNotifier {
    event_loop: Mutex<EventLoopProxy<MainThreadAction>>,
}

impl EventLoopNotifier {
    pub fn new(event_loop: EventLoopProxy<MainThreadAction>) -> Self {
        Self {
            event_loop: Mutex::new(event_loop),
        }
    }

    pub async fn request_redraw(&self) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::Redraw).await
    }

    pub async fn set_title(&self, title: String) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetTitle(title)).await
    }

    pub async fn set_size<S: Into<LogicalSize<u32>>>(&self, size: S) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetSize(size.into())).await
    }

    pub async fn set_is_maximized(&self, is_maximized: bool) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetIsMaximized(is_maximized)).await
    }

    pub async fn set_is_fullscreen(&self, is_fullscreen: bool) -> Result<(), EventLoopClosed> {
        self.send_action(MainThreadAction::SetIsFullscreen(is_fullscreen)).await
    }

    async fn send_action(&self, action: MainThreadAction) -> Result<(), EventLoopClosed> {
        Ok(self.event_loop.lock().await.send_event(action)?)
    }
}
