use crate::update::{
    Input,
    input::{MAX_KEYS, MAX_MOUSE_BUTTONS},
};
use alexandria::math::Vector2i;

impl Input {
    /// Create a new set of [`Input`]
    pub(in crate::update) fn new() -> Input {
        Input {
            keys: [false; MAX_KEYS],
            keys_down: [false; MAX_KEYS],
            keys_up: [false; MAX_KEYS],
            mouse_position: Vector2i::ZERO,
            mouse_buttons: [false; MAX_MOUSE_BUTTONS],
            mouse_buttons_down: [false; MAX_MOUSE_BUTTONS],
            mouse_buttons_up: [false; MAX_MOUSE_BUTTONS],
            mouse_wheel: 0,
        }
    }
}
