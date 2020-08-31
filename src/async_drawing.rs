use std::fmt::Debug;
use std::path::Path;

use serde::{Serialize, Deserialize};

use crate::ipc_protocol::ProtocolClient;
use crate::async_turtle::AsyncTurtle;
use crate::{Drawing, Point, Color, Event, ExportError};

/// Represents a size
///
/// A `Size` can be converted from either a tuple or array. These forms are often more ergonomic
/// than using the `Size` struct on its own. The [`set_size()`](struct.Drawing.html#method.set_size)
/// method accepts either form (without needing to use `into()`). See that method's documentation
/// for more information.
///
/// ```rust
/// # use turtle::Size;
/// assert_eq!(Size {width: 640, height: 480}, (640, 480).into());
/// assert_eq!(Size {width: 640, height: 480}, [640, 480].into());
/// ```
///
/// You can access the `width` and `height` fields directly on any `Size` struct.
///
/// ```rust
/// # use turtle::Size;
/// let size: Size = (800, 600).into();
/// assert_eq!(size.width, 800);
/// assert_eq!(size.height, 600);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Size {
    /// The width in pixels
    pub width: u32,
    /// The height in pixels
    pub height: u32,
}

impl From<(u32, u32)> for Size {
    fn from(size: (u32, u32)) -> Self {
        Self {
            width: size.0,
            height: size.1,
        }
    }
}

impl From<[u32; 2]> for Size {
    fn from(size: [u32; 2]) -> Self {
        Self {
            width: size[0],
            height: size[1],
        }
    }
}

pub struct AsyncDrawing {
    client: ProtocolClient,
}

impl From<Drawing> for AsyncDrawing {
    fn from(drawing: Drawing) -> Self {
        drawing.into_async()
    }
}

impl AsyncDrawing {
    pub async fn new() -> Self {
        // This needs to be called as close to the start of the program as possible. We call it
        // here since Drawing::new() or AsyncDrawing::new() are commonly called at the beginning
        // of many programs that use the turtle crate.
        crate::start();

        let client = ProtocolClient::new().await
            .expect("unable to create renderer client");
        Self {client}
    }

    pub async fn add_turtle(&mut self) -> AsyncTurtle {
        let client = self.client.split().await;
        AsyncTurtle::with_client(client).await
    }

    pub fn into_sync(self) -> Drawing {
        self.into()
    }

    pub async fn title(&self) -> String {
        self.client.drawing_title().await
    }

    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.client.drawing_set_title(title.into())
    }

    pub async fn background_color(&self) -> Color {
        self.client.drawing_background().await
    }

    pub fn set_background_color<C: Into<Color> + Copy + Debug>(&mut self, color: C) {
        let bg_color = color.into();
        assert!(
            bg_color.is_valid(),
            "Invalid color: {:?}. See the color module documentation for more information.",
            color
        );
        self.client.drawing_set_background(bg_color)
    }

    pub async fn center(&self) -> Point {
        self.client.drawing_center().await
    }

    pub fn set_center<P: Into<Point>>(&mut self, center: P) {
        let center = center.into();
        if !center.is_finite() {
            return;
        }
        self.client.drawing_set_center(center)
    }

    pub fn reset_center(&mut self) {
        self.client.drawing_reset_center()
    }

    pub async fn size(&self) -> Size {
        self.client.drawing_size().await
    }

    pub fn set_size<S: Into<Size>>(&mut self, size: S) {
        let size = size.into();
        assert!(size.width > 0 && size.height > 0, "The size of the drawing must be non-zero");

        self.client.drawing_set_size(size)
    }

    pub fn reset_size(&mut self) {
        self.client.drawing_reset_size()
    }

    pub async fn is_maximized(&self) -> bool {
        self.client.drawing_is_maximized().await
    }

    pub fn maximize(&mut self) {
        self.client.drawing_set_is_maximized(true)
    }

    pub fn unmaximize(&mut self) {
        self.client.drawing_set_is_maximized(false)
    }

    pub async fn is_fullscreen(&self) -> bool {
        self.client.drawing_is_fullscreen().await
    }

    pub fn enter_fullscreen(&mut self) {
        self.client.drawing_set_is_fullscreen(true)
    }

    pub fn exit_fullscreen(&mut self) {
        self.client.drawing_set_is_fullscreen(false)
    }

    pub fn clear(&mut self) {
        self.client.clear_all()
    }

    pub async fn poll_event(&mut self) -> Option<Event> {
        self.client.poll_event().await
    }

    pub async fn save_svg<P: AsRef<Path>>(&self, path: P) -> Result<(), ExportError> {
        self.client.export_svg(path.as_ref().to_path_buf()).await
    }

    pub async fn save_png<P: AsRef<Path>>(&self, path: P) -> Result<(), ExportError> {
        self.client.export_png(path.as_ref().to_path_buf()).await
    }

    //TODO: If we move to a shared memory architecture, we wouldn't need to make
    // any request here and thus would not need this method at all. We should
    // think things through before making this method public.
    /// # Stability
    ///
    /// **Warning:** This method exists because it is currently necessary to
    /// do some work asynchronously in order to print out a useful debug
    /// representation for this type. There is no async `Debug` trait. Please
    /// only use this method for debugging. It may be removed in a future
    /// release if we find a way to implement `Debug` trait for this type.
    pub async fn debug(&self) -> impl Debug {
        self.client.debug_drawing().await
    }
}
