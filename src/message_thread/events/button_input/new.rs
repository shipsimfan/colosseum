use crate::message_thread::RawInputButtonEvent;
use win32::HANDLE;

impl RawInputButtonEvent {
    /// Create a new [`RawInputButtonEvent`]
    pub fn new(device: HANDLE, button: u8, pressed: bool) -> Self {
        RawInputButtonEvent {
            device,
            button,
            pressed,
        }
    }
}
