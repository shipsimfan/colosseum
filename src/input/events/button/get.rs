use crate::input::{InputButtonEvent, InputDeviceId};

impl InputButtonEvent {
    /// Get the id of the device that generated this event
    pub fn id(&self) -> InputDeviceId {
        self.id
    }

    /// Get the button on the device that changed
    pub fn button(&self) -> u8 {
        self.button
    }

    /// Was the button pressed or released?
    pub fn pressed(&self) -> bool {
        self.pressed
    }
}
