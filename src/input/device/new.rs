use crate::input::{InputDevice, InputDeviceKind, InputDeviceMetadata};

impl InputDevice {
    /// Create a new [`InputDevice`]
    pub(crate) fn new(
        kind: InputDeviceKind,
        name: String,
        metadata: InputDeviceMetadata,
        num_buttons: u8,
        num_axes: u8,
    ) -> Self {
        InputDevice {
            kind,
            name,
            metadata,
            num_buttons,
            num_axes,
        }
    }
}
