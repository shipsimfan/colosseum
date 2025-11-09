use crate::input::{
    Input, InputAxisEvent, InputButtonEvent, InputDevice, InputDeviceId, StateTrackingInput,
    StateTrackingInputDevice,
};

impl Input for StateTrackingInput {
    fn new() -> Self {
        StateTrackingInput {
            input_devices: Vec::with_capacity(8),
            num_devices: 0,
        }
    }

    fn device_connected(&mut self, device: InputDevice) -> InputDeviceId {
        self.num_devices += 1;

        // Create device
        let device = StateTrackingInputDevice::new(device);

        // Search for slot
        for (i, slot) in self.input_devices.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(device);
                return i as u32;
            }
        }

        // No free slot, add it to the end
        let i = self.input_devices.len();
        self.input_devices.push(Some(device));
        i as u32
    }

    fn device_disconnected(&mut self, id: InputDeviceId) {
        self.num_devices -= 1;
        self.input_devices[id as usize] = None;
    }

    fn axis_event(&mut self, event: InputAxisEvent) {
        let device = self.input_devices[event.id() as usize].as_mut().unwrap();
        device.axis_event(event.axis(), event.value());
    }

    fn button_event(&mut self, event: InputButtonEvent) {
        let device = self.input_devices[event.id() as usize].as_mut().unwrap();
        device.button_event(event.button(), event.pressed());
    }

    fn frame(&mut self) {
        for device in &mut self.input_devices {
            if let Some(device) = device {
                device.frame();
            }
        }
    }
}
