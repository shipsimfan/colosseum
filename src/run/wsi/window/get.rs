use crate::{InputEvent, Window};
use alexandria::math::Vector2u;

impl Window {
    /// Gets the current size of the window
    pub fn size(&self) -> Vector2u {
        self.shared.size()
    }

    /// Get the next input event
    pub fn next_input(&self) -> Option<InputEvent> {
        self.inputs.try_recv().ok()
    }
}
