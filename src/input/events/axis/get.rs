use crate::input::{InputAxisEvent, InputDeviceId};

impl InputAxisEvent {
    /// Get the id of the device that generated this event
    pub fn id(&self) -> InputDeviceId {
        self.id
    }

    /// Get the axis of the device that was updated
    pub fn axis(&self) -> u8 {
        self.axis
    }

    /// Get the associated axis value
    ///
    /// If this comes from a mouse, it is either the mouse delta if the mouse is locked, or the
    /// absolute mouse position if it is unlocked.
    ///
    /// If this does not come from a mouse, it is the axis value normalized between -1.0 and 1.0.
    pub fn value(&self) -> f32 {
        self.value
    }
}
