use crate::input::{KeyCode, StateTrackingInput, StateTrackingInputDevice};

impl StateTrackingInput {
    /// Get the number of registered devices
    pub fn num_devices(&self) -> usize {
        self.num_devices
    }

    /// Get an iterator over the devices with their IDs
    pub fn devices(&self) -> impl Iterator<Item = (usize, &StateTrackingInputDevice)> {
        self.input_devices
            .iter()
            .enumerate()
            .filter_map(|(id, device)| device.as_ref().map(|device| (id, device)))
    }

    /// Get a device at `index`
    pub fn device(&self, index: usize) -> &StateTrackingInputDevice {
        self.input_devices[index].as_ref().unwrap()
    }

    /// Get if a `key` is currently pressed
    pub fn key(&self, key: KeyCode) -> bool {
        self.input_devices[0]
            .as_ref()
            .unwrap()
            .is_pressed(key as usize)
    }

    /// Get if a `key` was pressed this frame
    pub fn key_down(&self, key: KeyCode) -> bool {
        self.input_devices[0]
            .as_ref()
            .unwrap()
            .is_button_down(key as usize)
    }

    /// Get if a `key` was released this frame
    pub fn key_up(&self, key: KeyCode) -> bool {
        self.input_devices[0]
            .as_ref()
            .unwrap()
            .is_button_up(key as usize)
    }
}
