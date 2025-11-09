use win32::HANDLE;

mod get;
mod new;

/// A raw input button event
pub(in crate::message_thread) struct RawInputButtonEvent {
    /// The handle to the input device
    device: HANDLE,

    /// The button that changed
    ///
    /// If this comes from a keyboard, this can be translated to a [`crate::input::Keycode`]
    button: u8,

    /// True if the button was pressed, false if it was released
    pressed: bool,
}

unsafe impl Send for RawInputButtonEvent {}
