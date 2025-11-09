use crate::input::{InputButtonEvent, InputDeviceId};

impl InputButtonEvent {
    /// Create a new [`InputButtonEvent`]
    pub(crate) fn new(id: InputDeviceId, button: u8, pressed: bool) -> Self {
        InputButtonEvent {
            id,
            button,
            pressed,
        }
    }
}
