use tokio::sync::{
    Mutex,
    mpsc::{self, error::TryRecvError},
};
use glutin::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{self as glutin_event, WindowEvent, KeyboardInput},
};

use crate::ipc_protocol::{
    ServerConnection,
    ServerResponse,
};
use crate::renderer_client::ClientId;
use crate::{
    Point,
    Event,
    event::{PressedState, Key, MouseButton},
};

use super::HandlerError;
use super::super::coords::ScreenPoint;

/// An event and some context around the state when it was sent
///
/// This helps ensure that events are interpreted as they should be when they were first sent, not
/// based on the current values when the event was received/processed (since that may happen much
/// later.)
#[derive(Debug)]
pub struct EventContext {
    /// The event being sent
    pub event: Event,
    /// The DPI scale factor
    pub scale_factor: f64,
    /// The center of the drawing
    pub center: Point,
    /// The center of the framebuffer in screen coordinates
    pub fb_center: ScreenPoint,
}

pub(crate) async fn poll_event(
    conn: &ServerConnection,
    client_id: ClientId,
    events_receiver: &Mutex<mpsc::UnboundedReceiver<WindowEvent>>,
) -> Result<(), HandlerError> {
    let mut events_receiver = events_receiver.lock().await;

    let event = loop {
        match events_receiver.try_recv() {
            Ok(event) => if let Some(event) = from_window_event(event) {
                break Some(event);
            },
            Err(TryRecvError::Empty) => break None,
            // The main thread must have ended so no more events will be sent ever
            Err(TryRecvError::Closed) => return Ok(()),
        }
    };

    conn.send(client_id, ServerResponse::Event(event)).await?;

    Ok(())
}

/// Returns `None` if the input event is not a supported variant of `Event`
fn from_window_event(event: WindowEvent, scale_factor: f64, ) -> Option<Event> {
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
            let LogicalPosition {x, y} = ScreenPoint::from(position).to_logical(scale_factor, center, fb_center);
            Some(Event::MouseMove {x, y})
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

fn from_state(state: glutin_event::ElementState) -> PressedState {
    match state {
        glutin_event::ElementState::Pressed => PressedState::Pressed,
        glutin_event::ElementState::Released => PressedState::Released,
    }
}

fn from_keycode(key: glutin_event::VirtualKeyCode) -> Option<Key> {
    use glutin_event::VirtualKeyCode::*;
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

        Add => Key::Plus,
        Apostrophe => Key::Apostrophe,
        At => Key::At,
        Backslash => Key::Backslash,
        Colon => Key::Colon,
        Comma => Key::Comma,
        Decimal => Key::Decimal,
        Divide => Key::Divide,
        Equals => Key::Equals,
        Grave => Key::Backtick,
        LBracket => Key::LeftBracket,
        Minus => Key::Minus,
        Multiply => Key::Multiply,
        NumpadComma => Key::NumpadComma,
        NumpadEnter => Key::NumpadEnter,
        NumpadEquals => Key::NumpadEquals,
        Period => Key::Period,
        RBracket => Key::RightBracket,
        Semicolon => Key::Semicolon,
        Slash => Key::Slash,
        Subtract => Key::Minus,
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

fn from_button(button: glutin_event::MouseButton) -> Option<MouseButton> {
    use glutin_event::MouseButton::*;
    match button {
        Left => Some(MouseButton::LeftButton),
        Middle => Some(MouseButton::MiddleButton),
        Right => Some(MouseButton::RightButton),
        Other(_) => None,
    }
}
