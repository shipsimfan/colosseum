use crate::message_thread::RawInputButtonEvent;
use win32::HANDLE;

impl RawInputButtonEvent {
    /// Get the device that emitted this event
    pub fn device(&self) -> HANDLE {
        self.device
    }

    /// Get the button on the device that was pressed or released
    pub fn button(&self) -> u8 {
        self.button
    }

    /// Was the button pressed or released?
    pub fn pressed(&self) -> bool {
        self.pressed
    }
}
