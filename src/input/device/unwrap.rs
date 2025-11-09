use crate::input::{InputDevice, InputDeviceMetadata};

impl InputDevice {
    /// Consumes this [`InputDevice`] and returns owned versions of the metadata describing this device
    pub fn unwrap_metadata(self) -> (String, InputDeviceMetadata) {
        (self.name, self.metadata)
    }
}
