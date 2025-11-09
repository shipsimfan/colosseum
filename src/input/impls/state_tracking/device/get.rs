use crate::input::{InputDeviceKind, InputDeviceMetadata, StateTrackingInputDevice};

impl StateTrackingInputDevice {
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

    /// Get the number of buttons on the input device
    pub fn num_buttons(&self) -> usize {
        self.button_states.len()
    }

    /// Is `button` currently pressed?
    pub fn is_pressed(&self, button: usize) -> bool {
        self.button_states[button]
    }

    /// Was `button` pressed this frame?
    pub fn is_button_down(&self, button: usize) -> bool {
        self.button_down_states[button]
    }

    /// Was `button` released this frame?
    pub fn is_button_up(&self, button: usize) -> bool {
        self.button_up_states[button]
    }

    /// Get the number of axes on the input device
    pub fn num_axes(&self) -> usize {
        self.axis_states.len()
    }

    /// Get the current value of `axis`
    pub fn axis(&self, axis: usize) -> f32 {
        self.axis_states[axis]
    }
}
