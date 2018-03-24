//! Event handling (mouse, keyboard, controller, touch screen, etc.)
//!
//! See [`Event`](enum.Event.html) for more information.

use piston_window::{
    Event as PistonEvent,
    Input, ButtonArgs,
    ButtonState,
    Button,
    Motion,
    Key as PistonKey,
    MouseButton as PistonMouseButton,
    ControllerButton as PistonControllerButton,
    ControllerAxisArgs,
    TouchArgs,
    Touch as TouchState,
};

use {Point};

/// Possible events returned from [`Drawing::poll_event()`](../struct.Drawing.html#method.poll_event).
///
/// Events are used to make programs more interactive.
/// See that method's documentation for more information about how to use events.
///
/// This type is meant to provide a simplified model of `piston_window`'s `Event` type.
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
    #[allow(missing_docs)]
    MouseMove {x: f64, y: f64},
    /// Sent when the mouse is scrolled. Only sent when the mouse is over the window.
    /// `x` and `y` are in scroll ticks.
    #[allow(missing_docs)]
    MouseScroll {x: f64, y: f64},

    /// The start of touch, for example a finger pressed down on a touch screen.
    TouchStart(Touch),
    /// The move of touch, for example a finger moving while touching a touch screen.
    TouchMove(Touch),
    /// The end of touch, for example taking a finger away from a touch screen.
    TouchEnd(Touch),
    /// The cancel of touch, for example the window loses focus.
    TouchCancel(Touch),

    /// Sent when the window gets resized
    #[allow(missing_docs)]
    WindowResized {width: u32, height: u32},

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

/// Attempts to convert a piston Event to our event type
pub(crate) fn from_piston_event<F>(event: &PistonEvent, to_local_coords: F) -> Option<Event>
    where F: FnOnce(Point) -> Point {
    use self::Event::*;

    let input_event = match *event {
        PistonEvent::Input(ref input_event) => input_event,
        _ => return None,
    };

    Some(match *input_event {
        Input::Button(ButtonArgs {state, button, scancode: _}) => match state {
            ButtonState::Press => match button {
                Button::Keyboard(key) => KeyPressed(key.into()),
                Button::Mouse(button) => MouseButtonPressed(button.into()),
                Button::Controller(button) => ControllerButtonPressed(button.into()),
            },
            ButtonState::Release => match button {
                Button::Keyboard(key) => KeyReleased(key.into()),
                Button::Mouse(button) => MouseButtonReleased(button.into()),
                Button::Controller(button) => ControllerButtonReleased(button.into()),
            },
        },
        Input::Move(motion) => match motion {
            Motion::MouseCursor(x, y) => {
                let local = to_local_coords(Point {x, y});
                MouseMove {x: local.x, y: local.y}
            },
            // Ignored in favor of MouseCursor
            Motion::MouseRelative(..) => return None,
            Motion::MouseScroll(x, y) => MouseScroll {x, y},
            Motion::ControllerAxis(axis) => ControllerAxisChange(axis.into()),
            Motion::Touch(touch) => touch.into(),
        },
        // Ignored because this value doesn't produce text reliably for all keys
        // (especially when ctrl is pressed)
        Input::Text(_) => return None,
        Input::Resize(width, height) => WindowResized {width, height},
        Input::Focus(focused) => WindowFocused(focused),
        Input::Cursor(cursor) => WindowCursor(cursor),
        Input::Close(_) => WindowClosed,
    })
}

impl From<TouchArgs> for Event {
    fn from(args: TouchArgs) -> Self {
        match args.touch {
            TouchState::Start => Event::TouchStart(args.into()),
            TouchState::Move => Event::TouchMove(args.into()),
            TouchState::End => Event::TouchEnd(args.into()),
            TouchState::Cancel => Event::TouchCancel(args.into()),
        }
    }
}

/// Touch information
///
/// The `id` might be reused for different touches that do not overlap in time.
///
/// * Coordinates are normalized to support both touch screens and trackpads
/// * Supports both 2D and 3D touch
/// * The pressure direction vector should have maximum length 1
///
/// For 2D touch the pressure is pointed towards the z direction.
/// Use `.pressure()` to get the pressure magnitude.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Touch {
    /// A unique identifier for touch device.
    pub device: i64,
    /// A unique identifier for touch event.
    pub id: i64,
    /// The x coordinate of the touch position, normalized 0..1.
    pub x: f64,
    /// The y coordinate of the touch position, normalized 0..1.
    pub y: f64,
    /// The z coordinate of the touch position, normalized 0..1.
    pub z: f64,
    /// The x coordinate of the touch pressure direction.
    pub px: f64,
    /// The y coordinate of the touch pressure direction.
    pub py: f64,
    /// The z coordinate of the touch pressure direction.
    pub pz: f64,
    /// Whether the touch is in 3D.
    pub is_3d: bool,
}

impl From<TouchArgs> for Touch {
    fn from(TouchArgs {device, id, x, y, z, px, py, pz, is_3d, touch: _}: TouchArgs) -> Self {
        Self {device, id, x, y, z, px, py, pz, is_3d}
    }
}

impl Touch {
    /// The position of the touch in 2D.
    pub fn position(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    /// The position of the touch in 3D.
    pub fn position_3d(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// The pressure magnitude, normalized 0..1.
    pub fn pressure(&self) -> f64 {
        (self.px * self.px + self.py * self.py + self.pz * self.pz).sqrt()
    }

    /// The pressure vector in 3D.
    pub fn pressure_3d(&self) -> [f64; 3] {
        [self.px, self.py, self.pz]
    }
}

/// Represents a keyboard key
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    Unknown,
    Backspace,
    Tab,
    Return,
    Escape,
    Space,
    Exclaim,
    Quotedbl,
    Hash,
    Dollar,
    Percent,
    Ampersand,
    Quote,
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    Colon,
    Semicolon,
    Less,
    Equals,
    Greater,
    Question,
    At,
    LeftBracket,
    Backslash,
    RightBracket,
    Caret,
    Underscore,
    Backquote,
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
    Delete,
    CapsLock,
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
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLockClear,
    NumPadDivide,
    NumPadMultiply,
    NumPadMinus,
    NumPadPlus,
    NumPadEnter,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPad0,
    NumPadPeriod,
    Application,
    Power,
    NumPadEquals,
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
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    NumPadComma,
    NumPadEqualsAS400,
    AltErase,
    Sysreq,
    Cancel,
    Clear,
    Prior,
    Return2,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,
    NumPad00,
    NumPad000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubUnit,
    NumPadLeftParen,
    NumPadRightParen,
    NumPadLeftBrace,
    NumPadRightBrace,
    NumPadTab,
    NumPadBackspace,
    NumPadA,
    NumPadB,
    NumPadC,
    NumPadD,
    NumPadE,
    NumPadF,
    NumPadXor,
    NumPadPower,
    NumPadPercent,
    NumPadLess,
    NumPadGreater,
    NumPadAmpersand,
    NumPadDblAmpersand,
    NumPadVerticalBar,
    NumPadDblVerticalBar,
    NumPadColon,
    NumPadHash,
    NumPadSpace,
    NumPadAt,
    NumPadExclam,
    NumPadMemStore,
    NumPadMemRecall,
    NumPadMemClear,
    NumPadMemAdd,
    NumPadMemSubtract,
    NumPadMemMultiply,
    NumPadMemDivide,
    NumPadPlusMinus,
    NumPadClear,
    NumPadClearEntry,
    NumPadBinary,
    NumPadOctal,
    NumPadDecimal,
    NumPadHexadecimal,
    LCtrl,
    LShift,
    LAlt,
    LGui,
    RCtrl,
    RShift,
    RAlt,
    RGui,
    Mode,
    AudioNext,
    AudioPrev,
    AudioStop,
    AudioPlay,
    AudioMute,
    MediaSelect,
    Www,
    Mail,
    Calculator,
    Computer,
    AcSearch,
    AcHome,
    AcBack,
    AcForward,
    AcStop,
    AcRefresh,
    AcBookmarks,
    BrightnessDown,
    BrightnessUp,
    DisplaySwitch,
    KbdIllumToggle,
    KbdIllumDown,
    KbdIllumUp,
    Eject,
    Sleep,
}

impl From<PistonKey> for Key {
    fn from(key: PistonKey) -> Self {
        match key {
            PistonKey::Unknown => Key::Unknown,
            PistonKey::Backspace => Key::Backspace,
            PistonKey::Tab => Key::Tab,
            PistonKey::Return => Key::Return,
            PistonKey::Escape => Key::Escape,
            PistonKey::Space => Key::Space,
            PistonKey::Exclaim => Key::Exclaim,
            PistonKey::Quotedbl => Key::Quotedbl,
            PistonKey::Hash => Key::Hash,
            PistonKey::Dollar => Key::Dollar,
            PistonKey::Percent => Key::Percent,
            PistonKey::Ampersand => Key::Ampersand,
            PistonKey::Quote => Key::Quote,
            PistonKey::LeftParen => Key::LeftParen,
            PistonKey::RightParen => Key::RightParen,
            PistonKey::Asterisk => Key::Asterisk,
            PistonKey::Plus => Key::Plus,
            PistonKey::Comma => Key::Comma,
            PistonKey::Minus => Key::Minus,
            PistonKey::Period => Key::Period,
            PistonKey::Slash => Key::Slash,
            PistonKey::D0 => Key::D0,
            PistonKey::D1 => Key::D1,
            PistonKey::D2 => Key::D2,
            PistonKey::D3 => Key::D3,
            PistonKey::D4 => Key::D4,
            PistonKey::D5 => Key::D5,
            PistonKey::D6 => Key::D6,
            PistonKey::D7 => Key::D7,
            PistonKey::D8 => Key::D8,
            PistonKey::D9 => Key::D9,
            PistonKey::Colon => Key::Colon,
            PistonKey::Semicolon => Key::Semicolon,
            PistonKey::Less => Key::Less,
            PistonKey::Equals => Key::Equals,
            PistonKey::Greater => Key::Greater,
            PistonKey::Question => Key::Question,
            PistonKey::At => Key::At,
            PistonKey::LeftBracket => Key::LeftBracket,
            PistonKey::Backslash => Key::Backslash,
            PistonKey::RightBracket => Key::RightBracket,
            PistonKey::Caret => Key::Caret,
            PistonKey::Underscore => Key::Underscore,
            PistonKey::Backquote => Key::Backquote,
            PistonKey::A => Key::A,
            PistonKey::B => Key::B,
            PistonKey::C => Key::C,
            PistonKey::D => Key::D,
            PistonKey::E => Key::E,
            PistonKey::F => Key::F,
            PistonKey::G => Key::G,
            PistonKey::H => Key::H,
            PistonKey::I => Key::I,
            PistonKey::J => Key::J,
            PistonKey::K => Key::K,
            PistonKey::L => Key::L,
            PistonKey::M => Key::M,
            PistonKey::N => Key::N,
            PistonKey::O => Key::O,
            PistonKey::P => Key::P,
            PistonKey::Q => Key::Q,
            PistonKey::R => Key::R,
            PistonKey::S => Key::S,
            PistonKey::T => Key::T,
            PistonKey::U => Key::U,
            PistonKey::V => Key::V,
            PistonKey::W => Key::W,
            PistonKey::X => Key::X,
            PistonKey::Y => Key::Y,
            PistonKey::Z => Key::Z,
            PistonKey::Delete => Key::Delete,
            PistonKey::CapsLock => Key::CapsLock,
            PistonKey::F1 => Key::F1,
            PistonKey::F2 => Key::F2,
            PistonKey::F3 => Key::F3,
            PistonKey::F4 => Key::F4,
            PistonKey::F5 => Key::F5,
            PistonKey::F6 => Key::F6,
            PistonKey::F7 => Key::F7,
            PistonKey::F8 => Key::F8,
            PistonKey::F9 => Key::F9,
            PistonKey::F10 => Key::F10,
            PistonKey::F11 => Key::F11,
            PistonKey::F12 => Key::F12,
            PistonKey::PrintScreen => Key::PrintScreen,
            PistonKey::ScrollLock => Key::ScrollLock,
            PistonKey::Pause => Key::Pause,
            PistonKey::Insert => Key::Insert,
            PistonKey::Home => Key::Home,
            PistonKey::PageUp => Key::PageUp,
            PistonKey::End => Key::End,
            PistonKey::PageDown => Key::PageDown,
            PistonKey::Right => Key::Right,
            PistonKey::Left => Key::Left,
            PistonKey::Down => Key::Down,
            PistonKey::Up => Key::Up,
            PistonKey::NumLockClear => Key::NumLockClear,
            PistonKey::NumPadDivide => Key::NumPadDivide,
            PistonKey::NumPadMultiply => Key::NumPadMultiply,
            PistonKey::NumPadMinus => Key::NumPadMinus,
            PistonKey::NumPadPlus => Key::NumPadPlus,
            PistonKey::NumPadEnter => Key::NumPadEnter,
            PistonKey::NumPad1 => Key::NumPad1,
            PistonKey::NumPad2 => Key::NumPad2,
            PistonKey::NumPad3 => Key::NumPad3,
            PistonKey::NumPad4 => Key::NumPad4,
            PistonKey::NumPad5 => Key::NumPad5,
            PistonKey::NumPad6 => Key::NumPad6,
            PistonKey::NumPad7 => Key::NumPad7,
            PistonKey::NumPad8 => Key::NumPad8,
            PistonKey::NumPad9 => Key::NumPad9,
            PistonKey::NumPad0 => Key::NumPad0,
            PistonKey::NumPadPeriod => Key::NumPadPeriod,
            PistonKey::Application => Key::Application,
            PistonKey::Power => Key::Power,
            PistonKey::NumPadEquals => Key::NumPadEquals,
            PistonKey::F13 => Key::F13,
            PistonKey::F14 => Key::F14,
            PistonKey::F15 => Key::F15,
            PistonKey::F16 => Key::F16,
            PistonKey::F17 => Key::F17,
            PistonKey::F18 => Key::F18,
            PistonKey::F19 => Key::F19,
            PistonKey::F20 => Key::F20,
            PistonKey::F21 => Key::F21,
            PistonKey::F22 => Key::F22,
            PistonKey::F23 => Key::F23,
            PistonKey::F24 => Key::F24,
            PistonKey::Execute => Key::Execute,
            PistonKey::Help => Key::Help,
            PistonKey::Menu => Key::Menu,
            PistonKey::Select => Key::Select,
            PistonKey::Stop => Key::Stop,
            PistonKey::Again => Key::Again,
            PistonKey::Undo => Key::Undo,
            PistonKey::Cut => Key::Cut,
            PistonKey::Copy => Key::Copy,
            PistonKey::Paste => Key::Paste,
            PistonKey::Find => Key::Find,
            PistonKey::Mute => Key::Mute,
            PistonKey::VolumeUp => Key::VolumeUp,
            PistonKey::VolumeDown => Key::VolumeDown,
            PistonKey::NumPadComma => Key::NumPadComma,
            PistonKey::NumPadEqualsAS400 => Key::NumPadEqualsAS400,
            PistonKey::AltErase => Key::AltErase,
            PistonKey::Sysreq => Key::Sysreq,
            PistonKey::Cancel => Key::Cancel,
            PistonKey::Clear => Key::Clear,
            PistonKey::Prior => Key::Prior,
            PistonKey::Return2 => Key::Return2,
            PistonKey::Separator => Key::Separator,
            PistonKey::Out => Key::Out,
            PistonKey::Oper => Key::Oper,
            PistonKey::ClearAgain => Key::ClearAgain,
            PistonKey::CrSel => Key::CrSel,
            PistonKey::ExSel => Key::ExSel,
            PistonKey::NumPad00 => Key::NumPad00,
            PistonKey::NumPad000 => Key::NumPad000,
            PistonKey::ThousandsSeparator => Key::ThousandsSeparator,
            PistonKey::DecimalSeparator => Key::DecimalSeparator,
            PistonKey::CurrencyUnit => Key::CurrencyUnit,
            PistonKey::CurrencySubUnit => Key::CurrencySubUnit,
            PistonKey::NumPadLeftParen => Key::NumPadLeftParen,
            PistonKey::NumPadRightParen => Key::NumPadRightParen,
            PistonKey::NumPadLeftBrace => Key::NumPadLeftBrace,
            PistonKey::NumPadRightBrace => Key::NumPadRightBrace,
            PistonKey::NumPadTab => Key::NumPadTab,
            PistonKey::NumPadBackspace => Key::NumPadBackspace,
            PistonKey::NumPadA => Key::NumPadA,
            PistonKey::NumPadB => Key::NumPadB,
            PistonKey::NumPadC => Key::NumPadC,
            PistonKey::NumPadD => Key::NumPadD,
            PistonKey::NumPadE => Key::NumPadE,
            PistonKey::NumPadF => Key::NumPadF,
            PistonKey::NumPadXor => Key::NumPadXor,
            PistonKey::NumPadPower => Key::NumPadPower,
            PistonKey::NumPadPercent => Key::NumPadPercent,
            PistonKey::NumPadLess => Key::NumPadLess,
            PistonKey::NumPadGreater => Key::NumPadGreater,
            PistonKey::NumPadAmpersand => Key::NumPadAmpersand,
            PistonKey::NumPadDblAmpersand => Key::NumPadDblAmpersand,
            PistonKey::NumPadVerticalBar => Key::NumPadVerticalBar,
            PistonKey::NumPadDblVerticalBar => Key::NumPadDblVerticalBar,
            PistonKey::NumPadColon => Key::NumPadColon,
            PistonKey::NumPadHash => Key::NumPadHash,
            PistonKey::NumPadSpace => Key::NumPadSpace,
            PistonKey::NumPadAt => Key::NumPadAt,
            PistonKey::NumPadExclam => Key::NumPadExclam,
            PistonKey::NumPadMemStore => Key::NumPadMemStore,
            PistonKey::NumPadMemRecall => Key::NumPadMemRecall,
            PistonKey::NumPadMemClear => Key::NumPadMemClear,
            PistonKey::NumPadMemAdd => Key::NumPadMemAdd,
            PistonKey::NumPadMemSubtract => Key::NumPadMemSubtract,
            PistonKey::NumPadMemMultiply => Key::NumPadMemMultiply,
            PistonKey::NumPadMemDivide => Key::NumPadMemDivide,
            PistonKey::NumPadPlusMinus => Key::NumPadPlusMinus,
            PistonKey::NumPadClear => Key::NumPadClear,
            PistonKey::NumPadClearEntry => Key::NumPadClearEntry,
            PistonKey::NumPadBinary => Key::NumPadBinary,
            PistonKey::NumPadOctal => Key::NumPadOctal,
            PistonKey::NumPadDecimal => Key::NumPadDecimal,
            PistonKey::NumPadHexadecimal => Key::NumPadHexadecimal,
            PistonKey::LCtrl => Key::LCtrl,
            PistonKey::LShift => Key::LShift,
            PistonKey::LAlt => Key::LAlt,
            PistonKey::LGui => Key::LGui,
            PistonKey::RCtrl => Key::RCtrl,
            PistonKey::RShift => Key::RShift,
            PistonKey::RAlt => Key::RAlt,
            PistonKey::RGui => Key::RGui,
            PistonKey::Mode => Key::Mode,
            PistonKey::AudioNext => Key::AudioNext,
            PistonKey::AudioPrev => Key::AudioPrev,
            PistonKey::AudioStop => Key::AudioStop,
            PistonKey::AudioPlay => Key::AudioPlay,
            PistonKey::AudioMute => Key::AudioMute,
            PistonKey::MediaSelect => Key::MediaSelect,
            PistonKey::Www => Key::Www,
            PistonKey::Mail => Key::Mail,
            PistonKey::Calculator => Key::Calculator,
            PistonKey::Computer => Key::Computer,
            PistonKey::AcSearch => Key::AcSearch,
            PistonKey::AcHome => Key::AcHome,
            PistonKey::AcBack => Key::AcBack,
            PistonKey::AcForward => Key::AcForward,
            PistonKey::AcStop => Key::AcStop,
            PistonKey::AcRefresh => Key::AcRefresh,
            PistonKey::AcBookmarks => Key::AcBookmarks,
            PistonKey::BrightnessDown => Key::BrightnessDown,
            PistonKey::BrightnessUp => Key::BrightnessUp,
            PistonKey::DisplaySwitch => Key::DisplaySwitch,
            PistonKey::KbdIllumToggle => Key::KbdIllumToggle,
            PistonKey::KbdIllumDown => Key::KbdIllumDown,
            PistonKey::KbdIllumUp => Key::KbdIllumUp,
            PistonKey::Eject => Key::Eject,
            PistonKey::Sleep => Key::Sleep,
        }
    }
}

/// Represent a mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    /// Unknown mouse button.
    Unknown,
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Extra mouse button number 1.
    X1,
    /// Extra mouse button number 2.
    X2,
    /// Mouse button number 6.
    Button6,
    /// Mouse button number 7.
    Button7,
    /// Mouse button number 8.
    Button8,
}

impl From<PistonMouseButton> for MouseButton {
    fn from(button: PistonMouseButton) -> Self {
        match button {
            PistonMouseButton::Unknown => MouseButton::Unknown,
            PistonMouseButton::Left => MouseButton::Left,
            PistonMouseButton::Right => MouseButton::Right,
            PistonMouseButton::Middle => MouseButton::Middle,
            PistonMouseButton::X1 => MouseButton::X1,
            PistonMouseButton::X2 => MouseButton::X2,
            PistonMouseButton::Button6 => MouseButton::Button6,
            PistonMouseButton::Button7 => MouseButton::Button7,
            PistonMouseButton::Button8 => MouseButton::Button8,
        }
    }
}

/// Components of a controller button event.
/// Not guaranteed consistent across backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ControllerButton {
    /// Which controller was the button on.
    pub id: i32,
    /// Which button was pressed.
    pub button: u8,
}

impl From<PistonControllerButton> for ControllerButton {
    fn from(PistonControllerButton {id, button}: PistonControllerButton) -> Self {
        Self {id, button}
    }
}

/// Components of a controller axis move event.
/// Not guaranteed consistent across backends.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ControllerAxis {
    /// Which controller moved.
    pub id: i32,
    /// The axis that moved.
    pub axis: u8,
    /// Position of the controller. Usually [-1.0, 1.0], though backends may use
    /// a different range for various devices.
    pub position: f64,
}

impl From<ControllerAxisArgs> for ControllerAxis {
    fn from(ControllerAxisArgs {id, axis, position}: ControllerAxisArgs) -> Self {
        Self {id, axis, position}
    }
}
