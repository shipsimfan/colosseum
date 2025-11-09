mod device;

mod get;
mod input;

pub use device::StateTrackingInputDevice;

/// An input that tracks the state of inputs
pub struct StateTrackingInput {
    /// The devices that have been registered
    input_devices: Vec<Option<StateTrackingInputDevice>>,

    /// The current number of input devices
    num_devices: usize,
}
