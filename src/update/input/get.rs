use crate::{
    Key,
    update::{Input, input::mouse_button_index},
};
use alexandria::{input::MouseButton, math::Vector2i};

impl Input {
    /// Get if a key is currently pressed
    pub fn key(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    /// Get if a key was just pressed this frame
    pub fn key_down(&self, key: Key) -> bool {
        self.keys_down[key as usize]
    }

    /// Get if a key was just released this frame
    pub fn key_up(&self, key: Key) -> bool {
        self.keys_up[key as usize]
    }

    /// Get the current position of the mouse cursor
    pub fn mouse_position(&self) -> Vector2i {
        self.mouse_position
    }

    /// Get the state of a mouse button
    pub fn mouse_button(&self, button: MouseButton) -> bool {
        self.mouse_buttons[mouse_button_index(button)]
    }

    /// Get if a mouse button was just pressed this frame
    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down[mouse_button_index(button)]
    }

    /// Get if a mouse button was just released this frame
    pub fn mouse_button_up(&self, button: MouseButton) -> bool {
        self.mouse_buttons_up[mouse_button_index(button)]
    }

    /// Is the left mouse button currently pressed?
    pub fn left_mouse_button(&self) -> bool {
        self.mouse_button(MouseButton::Left)
    }

    /// Is the right mouse button currently pressed?
    pub fn right_mouse_button(&self) -> bool {
        self.mouse_button(MouseButton::Right)
    }

    /// Is the middle mouse button currently pressed?
    pub fn middle_mouse_button(&self) -> bool {
        self.mouse_button(MouseButton::Middle)
    }

    /// Is the left mouse button just pressed this frame?
    pub fn left_mouse_button_down(&self) -> bool {
        self.mouse_button_down(MouseButton::Left)
    }

    /// Is the right mouse button just pressed this frame?
    pub fn right_mouse_button_down(&self) -> bool {
        self.mouse_button_down(MouseButton::Right)
    }

    /// Is the middle mouse button just pressed this frame?
    pub fn middle_mouse_button_down(&self) -> bool {
        self.mouse_button_down(MouseButton::Middle)
    }

    /// Is the left mouse button just released this frame?
    pub fn left_mouse_button_up(&self) -> bool {
        self.mouse_button_up(MouseButton::Left)
    }

    /// Is the right mouse button just released this frame?
    pub fn right_mouse_button_up(&self) -> bool {
        self.mouse_button_up(MouseButton::Right)
    }

    /// Is the middle mouse button just released this frame?
    pub fn middle_mouse_button_up(&self) -> bool {
        self.mouse_button_up(MouseButton::Middle)
    }

    /// Get the current scroll wheel delta
    pub fn mouse_wheel(&self) -> i32 {
        self.mouse_wheel
    }
}
