use crate::Key;
use alexandria::input::MouseButton;

/// An event representing a change in input state, such as a key press or release
pub(crate) enum InputEvent {
    /// A key was pressed
    KeyDown { key: Key },

    /// A key was released
    KeyUp { key: Key },

    /// A mouse button was pressed
    MouseButtonDown { button: MouseButton },

    /// A mouse button was released
    MouseButtonUp { button: MouseButton },
}
