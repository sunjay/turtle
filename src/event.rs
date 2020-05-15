//! Event handling (mouse, keyboard, controller, touch screen, etc.)
//!
//! See [`Event`](enum.Event.html) for more information.
//!
//! # Unstable
//!
//! There are still many unanswered questions about the design of the events API in the turtle
//! crate. This module may change or be completely removed in the future. There will definitely
//! be *some* events API in the future, but it may end up looking different than it does today.

use serde::{Serialize, Deserialize};

/// Possible events returned from [`Drawing::poll_event()`](../struct.Drawing.html#method.poll_event).
///
/// Events are used to make programs more interactive. See that method's documentation for more
/// information about how to use events.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Sent when a keyboard key is pressed or released
    Key(Key, PressedState),

    /// Sent when a mouse button is pressed or released
    MouseButton(MouseButton, PressedState),

    /// Sent when the mouse is moving. Only sent when the mouse is over the window.
    /// `x` and `y` represent the new coordinates of where the mouse is currently.
    ///
    /// Coordinates are relative to the center of the window.
    MouseMove { x: f64, y: f64 },

    /// Sent when the mouse is scrolled. Only sent when the mouse is over the window.
    /// `x` and `y` are in scroll ticks.
    MouseScroll { x: f64, y: f64 },

    /// Sent when the window gets resized
    WindowResized { width: u32, height: u32 },

    /// Sent when the window focus changes
    ///
    /// The boolean value is true if the window is in focus.
    WindowFocused(bool),
    /// Sent when the cursor enters or leaves the window
    ///
    /// The boolean value is true if the cursor entered the window, and false if it left.
    WindowCursor(bool),
    /// Sent when the window is closed
    WindowClosed,
}

//TODO: Documentation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PressedState {
    Pressed,
    Released,
}

//TODO: Documentation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Key {
    /// The '1' key over the letters.
    Num1,
    /// The '2' key over the letters.
    Num2,
    /// The '3' key over the letters.
    Num3,
    /// The '4' key over the letters.
    Num4,
    /// The '5' key over the letters.
    Num5,
    /// The '6' key over the letters.
    Num6,
    /// The '7' key over the letters.
    Num7,
    /// The '8' key over the letters.
    Num8,
    /// The '9' key over the letters.
    Num9,
    /// The '0' key over the letters.
    Num0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1
    Esc,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Home,
    Delete,
    End,
    /// The PageDown (PgDn) key
    PageDown,
    /// The PageUp (PgUp) key
    PageUp,
    /// The backspace key, right over Enter/Return
    Backspace,
    /// The Enter/Return key, under Backspace
    Return,
    /// The spacebar key
    Space,

    /// The up arrow key
    UpArrow,
    /// The left arrow key
    LeftArrow,
    /// The right arrow key
    RightArrow,
    /// The down arrow key
    DownArrow,

    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,

    Apostrophe,
    At,
    Backslash,
    Backtick,
    Colon,
    Comma,
    Decimal,
    Divide,
    Equals,
    Minus,
    Multiply,
    Period,
    Plus,
    /// The left bracket `[` key
    LeftBracket,
    /// The left bracket `]` key
    RightBracket,
    Semicolon,
    Slash,
    Tab,
}

//TODO: Documentation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MouseButton {
    /// The left mouse button
    LeftButton,
    /// The middle mouse button
    MiddleButton,
    /// The right mouse button
    RightButton,
}
