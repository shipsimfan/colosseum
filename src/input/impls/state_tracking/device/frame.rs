use crate::input::StateTrackingInputDevice;

impl StateTrackingInputDevice {
    /// Called to save the buttons states from the previous frame
    pub(in crate::input::impls::state_tracking) fn frame(&mut self) {
        for down_state in &mut self.button_down_states {
            *down_state = false;
        }

        for up_state in &mut self.button_up_states {
            *up_state = false;
        }
    }
}
