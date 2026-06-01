use crate::Key;

/// An event representing a change in input state, such as a key press or release
pub(crate) enum InputEvent {
    /// A key was pressed
    KeyDown { key: Key },

    /// A key was released
    KeyUp { key: Key },
}
