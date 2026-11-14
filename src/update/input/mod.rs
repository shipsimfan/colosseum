use crate::Key;
use alexandria::{input::MouseButton, math::Vector2i};

mod get;
mod new;
mod reset;
mod set;
mod system;

/// The maximum number of keys that can be tracked for input
const MAX_KEYS: usize = Key::NpEnter as usize + 1;

/// The maximum number of mouse buttons that can be tracked for input
const MAX_MOUSE_BUTTONS: usize = 5;

/// The state of the inputs for the game
pub struct Input {
    /// The set of keyboard keys that are currently pressed
    keys: [bool; MAX_KEYS],

    /// The set of keyboard keys that were just pressed this frame
    keys_down: [bool; MAX_KEYS],

    /// The set of keyboard keys that were just released this frame
    keys_up: [bool; MAX_KEYS],

    /// The current position of the mouse cursor
    mouse_position: Vector2i,

    /// The set of mouse buttons that are currently pressed
    mouse_buttons: [bool; MAX_MOUSE_BUTTONS],

    /// The set of mouse buttons that were just pressed this frame
    mouse_buttons_down: [bool; MAX_MOUSE_BUTTONS],

    /// The set of mouse buttons that were just released this frame
    mouse_buttons_up: [bool; MAX_MOUSE_BUTTONS],

    /// The current scroll wheel delta
    mouse_wheel: i32,
}

/// Get the index of a mouse button in the internal arrays
fn mouse_button_index(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(other) => other as usize,
    }
}
