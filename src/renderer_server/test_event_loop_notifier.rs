use thiserror::Error;
use glutin::dpi::LogicalSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("event loop closed while messages were still being sent to it")]
pub struct EventLoopClosed;

/// An event loop notifier for use in tests that does nothing
#[derive(Debug)]
pub struct EventLoopNotifier {}

impl EventLoopNotifier {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn request_redraw(&self) -> Result<(), EventLoopClosed> {
        Ok(())
    }

    pub async fn set_title(&self, _title: String) -> Result<(), EventLoopClosed> {
        Ok(())
    }

    pub async fn set_size<S: Into<LogicalSize<u32>>>(&self, _size: S) -> Result<(), EventLoopClosed> {
        Ok(())
    }

    pub async fn set_is_maximized(&self, _is_maximized: bool) -> Result<(), EventLoopClosed> {
        Ok(())
    }

    pub async fn set_is_fullscreen(&self, _is_fullscreen: bool) -> Result<(), EventLoopClosed> {
        Ok(())
    }
}
