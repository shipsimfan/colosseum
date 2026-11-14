use crate::{
    Key,
    update::{Input, input::mouse_button_index},
};
use alexandria::{input::MouseButton, math::Vector2i};

impl Input {
    /// Add a key to the set of currently pressed keys
    pub(in crate::update::input) fn set_key_down(&mut self, key: Key) {
        if !self.key(key) {
            self.keys_down[key as usize] = true;
        }

        self.keys[key as usize] = true;
    }

    /// Remove a key from the set of currently pressed keys
    pub(in crate::update::input) fn set_key_up(&mut self, key: Key) {
        if self.key(key) {
            self.keys_up[key as usize] = true;
        }

        self.keys[key as usize] = false;
    }

    /// Set the current position of the mouse cursor
    pub(in crate::update::input) fn set_mouse_position(&mut self, position: Vector2i) {
        self.mouse_position = position;
    }

    /// Add a mouse button to the set of currently pressed mouse buttons
    pub(in crate::update::input) fn set_mouse_button_down(&mut self, button: MouseButton) {
        if !self.mouse_button(button) {
            self.mouse_buttons_down[mouse_button_index(button)] = true;
        }

        self.mouse_buttons[mouse_button_index(button)] = true;
    }

    /// Remove a mouse button from the set of currently pressed mouse buttons
    pub(in crate::update::input) fn set_mouse_button_up(&mut self, button: MouseButton) {
        if self.mouse_button(button) {
            self.mouse_buttons_up[mouse_button_index(button)] = true;
        }

        self.mouse_buttons[mouse_button_index(button)] = false;
    }

    /// Set the current scroll wheel delta
    pub(in crate::update::input) fn set_mouse_wheel(&mut self, delta: i32) {
        self.mouse_wheel = delta;
    }
}
