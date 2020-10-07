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
use glutin::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{self as glutin_event, ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
};

use crate::Point;

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
    MouseMove(Point),

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

impl Event {
    /// Returns `None` if the input event is not a supported variant of `Event`
    #[cfg_attr(any(feature = "test", test), allow(dead_code))]
    pub(crate) fn from_window_event(
        event: WindowEvent,
        scale_factor: f64,
        to_logical: impl FnOnce(PhysicalPosition<f64>) -> Point,
    ) -> Option<Self> {
        match event {
            WindowEvent::Resized(size) => {
                let LogicalSize {width, height} = size.to_logical(scale_factor);
                Some(Event::WindowResized {width, height})
            },

            WindowEvent::KeyboardInput {input: KeyboardInput {state, virtual_keycode, ..}, ..} => {
                Some(Event::Key(
                    Key::from_keycode(virtual_keycode?)?,
                    PressedState::from_state(state),
                ))
            },
            WindowEvent::CursorEntered {..} => Some(Event::WindowCursor(true)),
            WindowEvent::CursorLeft {..} => Some(Event::WindowCursor(false)),
            WindowEvent::CursorMoved {position, ..} => {
                Some(Event::MouseMove(to_logical(position)))
            },
            WindowEvent::MouseInput {state, button, ..} => Some(Event::MouseButton(
                MouseButton::from_button(button)?,
                PressedState::from_state(state),
            )),
            WindowEvent::Focused(focused) => Some(Event::WindowFocused(focused)),
            WindowEvent::Destroyed => Some(Event::WindowClosed),

            WindowEvent::Moved(_) |
            WindowEvent::CloseRequested |
            WindowEvent::DroppedFile(_) |
            WindowEvent::HoveredFile(_) |
            WindowEvent::HoveredFileCancelled |
            WindowEvent::ReceivedCharacter(_) |
            WindowEvent::ModifiersChanged(_) |
            WindowEvent::MouseWheel {..} |
            WindowEvent::TouchpadPressure {..} |
            WindowEvent::AxisMotion {..} |
            WindowEvent::Touch(_) |
            WindowEvent::ScaleFactorChanged {..} |
            WindowEvent::ThemeChanged(_) => None, // Not supported
        }
    }
}

//TODO: Documentation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PressedState {
    Pressed,
    Released,
}

impl PressedState {
    #[cfg_attr(any(feature = "test", test), allow(dead_code))]
    fn from_state(state: ElementState) -> PressedState {
        match state {
            ElementState::Pressed => PressedState::Pressed,
            ElementState::Released => PressedState::Released,
        }
    }
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

impl Key {
    #[cfg_attr(any(feature = "test", test), allow(dead_code))]
    fn from_keycode(key: VirtualKeyCode) -> Option<Self> {
        Some(match key {
            VirtualKeyCode::Key1 => Key::Num1,
            VirtualKeyCode::Key2 => Key::Num2,
            VirtualKeyCode::Key3 => Key::Num3,
            VirtualKeyCode::Key4 => Key::Num4,
            VirtualKeyCode::Key5 => Key::Num5,
            VirtualKeyCode::Key6 => Key::Num6,
            VirtualKeyCode::Key7 => Key::Num7,
            VirtualKeyCode::Key8 => Key::Num8,
            VirtualKeyCode::Key9 => Key::Num9,
            VirtualKeyCode::Key0 => Key::Num0,

            VirtualKeyCode::A => Key::A,
            VirtualKeyCode::B => Key::B,
            VirtualKeyCode::C => Key::C,
            VirtualKeyCode::D => Key::D,
            VirtualKeyCode::E => Key::E,
            VirtualKeyCode::F => Key::F,
            VirtualKeyCode::G => Key::G,
            VirtualKeyCode::H => Key::H,
            VirtualKeyCode::I => Key::I,
            VirtualKeyCode::J => Key::J,
            VirtualKeyCode::K => Key::K,
            VirtualKeyCode::L => Key::L,
            VirtualKeyCode::M => Key::M,
            VirtualKeyCode::N => Key::N,
            VirtualKeyCode::O => Key::O,
            VirtualKeyCode::P => Key::P,
            VirtualKeyCode::Q => Key::Q,
            VirtualKeyCode::R => Key::R,
            VirtualKeyCode::S => Key::S,
            VirtualKeyCode::T => Key::T,
            VirtualKeyCode::U => Key::U,
            VirtualKeyCode::V => Key::V,
            VirtualKeyCode::W => Key::W,
            VirtualKeyCode::X => Key::X,
            VirtualKeyCode::Y => Key::Y,
            VirtualKeyCode::Z => Key::Z,

            VirtualKeyCode::Escape => Key::Esc,

            VirtualKeyCode::F1 => Key::F1,
            VirtualKeyCode::F2 => Key::F2,
            VirtualKeyCode::F3 => Key::F3,
            VirtualKeyCode::F4 => Key::F4,
            VirtualKeyCode::F5 => Key::F5,
            VirtualKeyCode::F6 => Key::F6,
            VirtualKeyCode::F7 => Key::F7,
            VirtualKeyCode::F8 => Key::F8,
            VirtualKeyCode::F9 => Key::F9,
            VirtualKeyCode::F10 => Key::F10,
            VirtualKeyCode::F11 => Key::F11,
            VirtualKeyCode::F12 => Key::F12,
            VirtualKeyCode::F13 => Key::F13,
            VirtualKeyCode::F14 => Key::F14,
            VirtualKeyCode::F15 => Key::F15,
            VirtualKeyCode::F16 => Key::F16,
            VirtualKeyCode::F17 => Key::F17,
            VirtualKeyCode::F18 => Key::F18,
            VirtualKeyCode::F19 => Key::F19,
            VirtualKeyCode::F20 => Key::F20,
            VirtualKeyCode::F21 => Key::F21,
            VirtualKeyCode::F22 => Key::F22,
            VirtualKeyCode::F23 => Key::F23,
            VirtualKeyCode::F24 => Key::F24,

            VirtualKeyCode::Home => Key::Home,
            VirtualKeyCode::Delete => Key::Delete,
            VirtualKeyCode::End => Key::End,
            VirtualKeyCode::PageDown => Key::PageDown,
            VirtualKeyCode::PageUp => Key::PageUp,
            VirtualKeyCode::Back => Key::Backspace,
            VirtualKeyCode::Return => Key::Return,
            VirtualKeyCode::Space => Key::Space,

            VirtualKeyCode::Left => Key::LeftArrow,
            VirtualKeyCode::Up => Key::UpArrow,
            VirtualKeyCode::Right => Key::RightArrow,
            VirtualKeyCode::Down => Key::DownArrow,

            VirtualKeyCode::Numpad0 => Key::Numpad0,
            VirtualKeyCode::Numpad1 => Key::Numpad1,
            VirtualKeyCode::Numpad2 => Key::Numpad2,
            VirtualKeyCode::Numpad3 => Key::Numpad3,
            VirtualKeyCode::Numpad4 => Key::Numpad4,
            VirtualKeyCode::Numpad5 => Key::Numpad5,
            VirtualKeyCode::Numpad6 => Key::Numpad6,
            VirtualKeyCode::Numpad7 => Key::Numpad7,
            VirtualKeyCode::Numpad8 => Key::Numpad8,
            VirtualKeyCode::Numpad9 => Key::Numpad9,

            VirtualKeyCode::Apostrophe => Key::Apostrophe,
            VirtualKeyCode::At => Key::At,
            VirtualKeyCode::Backslash => Key::Backslash,
            VirtualKeyCode::Colon => Key::Colon,
            VirtualKeyCode::Comma => Key::Comma,
            VirtualKeyCode::Equals => Key::Equals,
            VirtualKeyCode::Grave => Key::Backtick,
            VirtualKeyCode::LBracket => Key::LeftBracket,
            VirtualKeyCode::Minus => Key::Minus,
            VirtualKeyCode::NumpadAdd => Key::Plus,
            VirtualKeyCode::NumpadComma => Key::NumpadComma,
            VirtualKeyCode::NumpadDecimal => Key::Decimal,
            VirtualKeyCode::NumpadDivide => Key::Divide,
            VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
            VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
            VirtualKeyCode::NumpadMultiply => Key::Multiply,
            VirtualKeyCode::NumpadSubtract => Key::Minus,
            VirtualKeyCode::Period => Key::Period,
            VirtualKeyCode::RBracket => Key::RightBracket,
            VirtualKeyCode::Semicolon => Key::Semicolon,
            VirtualKeyCode::Slash => Key::Slash,
            VirtualKeyCode::Tab => Key::Tab,

            // Unsupported keys (could be changed in the future)
            _ => return None,
        })
    }
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

impl MouseButton {
    #[cfg_attr(any(feature = "test", test), allow(dead_code))]
    fn from_button(button: glutin_event::MouseButton) -> Option<Self> {
        match button {
            glutin_event::MouseButton::Left => Some(MouseButton::LeftButton),
            glutin_event::MouseButton::Middle => Some(MouseButton::MiddleButton),
            glutin_event::MouseButton::Right => Some(MouseButton::RightButton),
            glutin_event::MouseButton::Other(_) => None,
        }
    }
}
