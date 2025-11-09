use crate::input::InputDeviceId;

mod get;

/// An axis of an input device changed
pub struct InputAxisEvent {
    /// The id of the device that generated this event
    id: InputDeviceId,

    /// The axis of the device that changed
    axis: u8,

    /// The associated axis value
    ///
    /// If this comes from a mouse, it is either the mouse delta if the mouse is locked, or the
    /// absolute mouse position if it is unlocked.
    ///
    /// If this does not come from a mouse, it is the axis value normalized between -1.0 and 1.0.
    value: f32,
}
