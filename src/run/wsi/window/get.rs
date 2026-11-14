use crate::{InputEvent, Window};
use alexandria::math::{Vector2i, Vector2u};

impl Window {
    /// Get the current position of the window
    pub fn position(&self) -> Vector2i {
        self.shared.position()
    }

    /// Gets the current size of the window
    pub fn size(&self) -> Vector2u {
        self.shared.size()
    }

    /// Get whether the window is currently fullscreen or not
    pub fn fullscreen(&self) -> bool {
        self.shared.fullscreen()
    }

    /// Get whether the window is currently maximized or not
    pub fn maximized(&self) -> bool {
        self.shared.maximized()
    }

    /// Get whether the window is currently focused or not
    pub fn focused(&self) -> bool {
        self.shared.focused()
    }

    /// Get the next input event
    pub(crate) fn next_input(&self) -> Option<InputEvent> {
        self.inputs.try_recv().ok()
    }

    /// Get the current position of the mouse cursor
    pub(crate) fn mouse_position(&self) -> Vector2i {
        self.shared.mouse_position()
    }

    /// Get the current scroll wheel delta
    pub(crate) fn mouse_wheel(&self) -> i32 {
        self.shared.mouse_wheel()
    }

    /// Should the window be closed?
    pub(crate) fn should_close(&self) -> bool {
        self.closing
    }
}
