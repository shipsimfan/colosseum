use crate::Key;

mod get;
mod new;
mod reset;
mod set;

/// The maximum number of keys that can be tracked for input
const MAX_KEYS: usize = Key::NpEnter as usize + 1;

/// The state of the inputs for the game
pub struct Inputs {
    /// The set of keyboard keys that are currently pressed
    keys: [bool; MAX_KEYS],

    /// The set of keyboard keys that were just pressed this frame
    keys_down: [bool; MAX_KEYS],

    /// The set of keyboard keys that were just released this frame
    keys_up: [bool; MAX_KEYS],
}
