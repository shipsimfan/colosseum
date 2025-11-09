mod id;
mod kind;
mod metadata;

mod get;
mod new;
mod unwrap;

pub use id::InputDeviceId;
pub use kind::InputDeviceKind;
pub use metadata::InputDeviceMetadata;

/// The description of an input device
pub struct InputDevice {
    /// The kind of input device this is
    kind: InputDeviceKind,

    /// A friendly name for the device
    name: String,

    /// Metadata to identify the specific kind of device
    metadata: InputDeviceMetadata,

    /// The number of buttons the device has
    num_buttons: u8,

    /// The number of axes the device has
    num_axes: u8,
}
