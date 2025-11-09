/// The kind of a device an input device is
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDeviceKind {
    /// The input device is a keyboard
    Keyboard,

    /// The input device is a mouse
    Mouse,

    /// The input device is an X-Box controller
    XBoxController,

    /// The input device is a non-X-Box controller
    OtherController,

    /// The input device is a steering wheel
    SteeringWheel,

    /// The input device is a joystick
    Joystick,

    /// The input device is something else
    Other,
}
