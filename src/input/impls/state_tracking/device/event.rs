use crate::input::StateTrackingInputDevice;

impl StateTrackingInputDevice {
    /// Update the state of a button
    pub(in crate::input::impls::state_tracking) fn button_event(
        &mut self,
        button: u8,
        pressed: bool,
    ) {
        self.button_states[button as usize] = pressed;
        if pressed {
            self.button_down_states[button as usize] = true;
        } else {
            self.button_up_states[button as usize] = true;
        }
    }

    /// Update the state of an axis
    pub(in crate::input::impls::state_tracking) fn axis_event(&mut self, axis: u8, value: f32) {
        self.axis_states[axis as usize] = value;
    }
}
