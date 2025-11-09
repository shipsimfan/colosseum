use crate::input::InputDeviceId;

mod get;
mod new;

/// A button of an input device was pressed or released
pub struct InputButtonEvent {
    /// The id of the device that generated this event
    id: InputDeviceId,

    /// The button that changed
    ///
    /// If this comes from a keyboard, this can be translated to a [`crate::input::Keycode`]
    button: u8,

    /// True if the button was pressed, false if it was released
    pressed: bool,
}
