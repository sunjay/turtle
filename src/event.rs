//! Event handling (mouse, keyboard, controller, touch screen, etc.)
//!
//! See [`Event`](enum.Event.html) for more information.

use serde::{Serialize, Deserialize};

/// Possible events returned from [`Drawing::poll_event()`](../struct.Drawing.html#method.poll_event).
///
/// Events are used to make programs more interactive. See that method's documentation for more
/// information about how to use events.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Sent when a keyboard key is pressed
    KeyPressed(Key),
    /// Sent when a keyboard key is released
    KeyReleased(Key),
    /// Sent when a mouse button is pressed
    MouseButtonPressed(MouseButton),
    /// Sent when a mouse button is released
    MouseButtonReleased(MouseButton),
    /// Sent when a controller button is pressed
    ControllerButtonPressed(ControllerButton),
    /// Sent when a controller button is released
    ControllerButtonReleased(ControllerButton),
    /// Sent when a controller axis (usually a joystick) is changed
    ControllerAxisChange(ControllerAxis),

    /// Sent when the mouse is moving. Only sent when the mouse is over the window.
    /// `x` and `y` represent the new coordinates of where the mouse is currently.
    ///
    /// Coordinates are relative to the center of the window.
    MouseMove { x: f64, y: f64 },
    /// Sent when the mouse is scrolled. Only sent when the mouse is over the window.
    /// `x` and `y` are in scroll ticks.
    MouseScroll { x: f64, y: f64 },

    /// Sent when a user touches the screen
    Touch(Touch),

    /// Sent when the window gets resized
    WindowResized { width: u32, height: u32 },

    /// Sent when the window focus changes
    ///
    /// The boolean value is true if the window is in focus.
    WindowFocused(bool),
    /// Sent when the window gains or loses the cursor.
    ///
    /// The boolean value is true if the window gained the cursor.
    WindowCursor(bool),
    /// Sent when the window is closed
    WindowClosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Key {
    //TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MouseButton {
    //TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ControllerButton {
    //TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ControllerAxis {
    //TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Touch {
    //TODO
}
