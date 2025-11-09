use crate::input::{InputDevice, InputDeviceKind, InputDeviceMetadata};

impl InputDevice {
    /// Get the kind of input device this is
    pub fn kind(&self) -> InputDeviceKind {
        self.kind
    }

    /// Get the name of the device
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the metadata to identify the kind of device this is
    pub fn metadata(&self) -> &InputDeviceMetadata {
        &self.metadata
    }

    /// The number of buttons on the input device
    pub fn num_buttons(&self) -> u8 {
        self.num_buttons
    }

    /// Get the number of axes on the input devicek
    pub fn num_axes(&self) -> u8 {
        self.num_axes
    }
}
