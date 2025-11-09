//! User input subsystem

mod device;
mod events;
mod impls;
mod key_code;

pub use device::*;
pub use events::*;
pub use impls::*;
pub use key_code::KeyCode;

/// A subsystem which can consume input events and present and interface for accessing them
pub trait Input {
    /// Create a new version of this [`Input`]
    fn new() -> Self;

    /// A new device has been connected
    ///
    /// Return a new ID to identify the input device for running
    fn device_connected(&mut self, device: InputDevice) -> InputDeviceId;

    /// A previously connected device has been disconnected
    ///
    /// Once the device has been disconnected, its ID can be re-used
    fn device_disconnected(&mut self, id: InputDeviceId);

    /// A button was pressed or released
    fn button_event(&mut self, event: InputButtonEvent);

    /// An axis was changed
    fn axis_event(&mut self, event: InputAxisEvent);

    /// Called when a frame ends and just before new input events are going to be given
    fn frame(&mut self);
}
