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
    event::{self as glutin_event, WindowEvent, KeyboardInput},
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
    fn from_state(state: glutin_event::ElementState) -> PressedState {
        match state {
            glutin_event::ElementState::Pressed => PressedState::Pressed,
            glutin_event::ElementState::Released => PressedState::Released,
        }
    }
}

//TODO: Documentation
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Key {
    /// The '1' key above the letters.
    Num1,
    /// The '2' key above the letters.
    Num2,
    /// The '3' key above the letters.
    Num3,
    /// The '4' key above the letters.
    Num4,
    /// The '5' key above the letters.
    Num5,
    /// The '6' key above the letters.
    Num6,
    /// The '7' key above the letters.
    Num7,
    /// The '8' key above the letters.
    Num8,
    /// The '9' key above the letters.
    Num9,
    /// The '0' key above the letters.
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
    /// The backspace key, right above Enter/Return
    Backspace,
    /// The Enter/Return key, below Backspace
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
    fn from_keycode(key: glutin_event::VirtualKeyCode) -> Option<Self> {
        use glutin_event::VirtualKeyCode::*;
        #[deny(unreachable_patterns, unused_variables)]
        Some(match key {
            Key1 => Key::Num1,
            Key2 => Key::Num2,
            Key3 => Key::Num3,
            Key4 => Key::Num4,
            Key5 => Key::Num5,
            Key6 => Key::Num6,
            Key7 => Key::Num7,
            Key8 => Key::Num8,
            Key9 => Key::Num9,
            Key0 => Key::Num0,

            A => Key::A,
            B => Key::B,
            C => Key::C,
            D => Key::D,
            E => Key::E,
            F => Key::F,
            G => Key::G,
            H => Key::H,
            I => Key::I,
            J => Key::J,
            K => Key::K,
            L => Key::L,
            M => Key::M,
            N => Key::N,
            O => Key::O,
            P => Key::P,
            Q => Key::Q,
            R => Key::R,
            S => Key::S,
            T => Key::T,
            U => Key::U,
            V => Key::V,
            W => Key::W,
            X => Key::X,
            Y => Key::Y,
            Z => Key::Z,

            Escape => Key::Esc,

            F1 => Key::F1,
            F2 => Key::F2,
            F3 => Key::F3,
            F4 => Key::F4,
            F5 => Key::F5,
            F6 => Key::F6,
            F7 => Key::F7,
            F8 => Key::F8,
            F9 => Key::F9,
            F10 => Key::F10,
            F11 => Key::F11,
            F12 => Key::F12,
            F13 => Key::F13,
            F14 => Key::F14,
            F15 => Key::F15,
            F16 => Key::F16,
            F17 => Key::F17,
            F18 => Key::F18,
            F19 => Key::F19,
            F20 => Key::F20,
            F21 => Key::F21,
            F22 => Key::F22,
            F23 => Key::F23,
            F24 => Key::F24,

            Home => Key::Home,
            Delete => Key::Delete,
            End => Key::End,
            PageDown => Key::PageDown,
            PageUp => Key::PageUp,
            Back => Key::Backspace,
            Return => Key::Return,
            Space => Key::Space,

            Left => Key::LeftArrow,
            Up => Key::UpArrow,
            Right => Key::RightArrow,
            Down => Key::DownArrow,

            Numpad0 => Key::Numpad0,
            Numpad1 => Key::Numpad1,
            Numpad2 => Key::Numpad2,
            Numpad3 => Key::Numpad3,
            Numpad4 => Key::Numpad4,
            Numpad5 => Key::Numpad5,
            Numpad6 => Key::Numpad6,
            Numpad7 => Key::Numpad7,
            Numpad8 => Key::Numpad8,
            Numpad9 => Key::Numpad9,

            Apostrophe => Key::Apostrophe,
            At => Key::At,
            Backslash => Key::Backslash,
            Colon => Key::Colon,
            Comma => Key::Comma,
            Equals => Key::Equals,
            Grave => Key::Backtick,
            LBracket => Key::LeftBracket,
            NumpadAdd | Plus => Key::Plus,
            NumpadComma => Key::NumpadComma,
            NumpadDecimal => Key::Decimal,
            NumpadDivide => Key::Divide,
            NumpadEnter => Key::NumpadEnter,
            NumpadEquals => Key::NumpadEquals,
            NumpadMultiply | Asterisk => Key::Multiply,
            NumpadSubtract | Minus => Key::Minus,
            Period => Key::Period,
            RBracket => Key::RightBracket,
            Semicolon => Key::Semicolon,
            Slash => Key::Slash,
            Tab => Key::Tab,

            // Unsupported keys (could be changed in the future)
            Snapshot |
            Scroll |
            Pause |
            Insert |
            Compose |
            Caret |
            Numlock |
            AbntC1 |
            AbntC2 |
            Apps |
            Ax |
            Calculator |
            Capital |
            Convert |
            Kana |
            Kanji |
            LAlt |
            LControl |
            LShift |
            LWin |
            Mail |
            MediaSelect |
            MediaStop |
            Mute |
            MyComputer |
            NavigateForward |
            NavigateBackward |
            NextTrack |
            NoConvert |
            OEM102 |
            PlayPause |
            Power |
            PrevTrack |
            RAlt |
            RControl |
            RShift |
            RWin |
            Sleep |
            Stop |
            Sysrq |
            Underline |
            Unlabeled |
            VolumeDown |
            VolumeUp |
            Wake |
            WebBack |
            WebFavorites |
            WebForward |
            WebHome |
            WebRefresh |
            WebSearch |
            WebStop |
            Yen |
            Copy |
            Paste |
            Cut => return None,
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
        use glutin_event::MouseButton::*;
        #[deny(unreachable_patterns, unused_variables)]
        match button {
            Left => Some(MouseButton::LeftButton),
            Middle => Some(MouseButton::MiddleButton),
            Right => Some(MouseButton::RightButton),
            Other(_) => None,
        }
    }
}
