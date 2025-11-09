use crate::input::{InputDevice, StateTrackingInputDevice};

impl StateTrackingInputDevice {
    /// Create a new [`StateTrackingInputDevice`]
    pub(in crate::input::impls::state_tracking) fn new(device: InputDevice) -> Self {
        let button_states = vec![false; device.num_buttons() as usize];
        let button_down_states = vec![false; device.num_buttons() as usize];
        let button_up_states = vec![false; device.num_buttons() as usize];
        let axis_states = vec![0.0; device.num_axes() as usize];

        let kind = device.kind();
        let (name, metadata) = device.unwrap_metadata();

        StateTrackingInputDevice {
            kind,
            name,
            metadata,
            button_states,
            button_down_states,
            button_up_states,
            axis_states,
        }
    }
}
