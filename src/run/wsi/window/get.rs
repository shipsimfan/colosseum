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

    /// Get the next input event
    pub(crate) fn next_input(&self) -> Option<InputEvent> {
        self.inputs.try_recv().ok()
    }

    /// Should the window be closed?
    pub(crate) fn should_close(&self) -> bool {
        self.closing
    }
}
