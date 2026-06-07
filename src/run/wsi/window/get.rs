use crate::{Error, InputEvent, Result, Window};
use alexandria::math::{Vector2i, Vector2u};

impl Window {
    /// Get the current position of the window
    pub fn position(&self) -> Vector2i {
        self.shared.position()
    }

    /// Gets the current size of the window
    pub(in crate::run) fn size(&self) -> Vector2u {
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
    pub fn next_input(&self) -> Option<InputEvent> {
        self.inputs.try_recv().ok()
    }

    /// Wait for the restored notify to be signalled
    pub(in crate::run) fn wait_for_restore(&self) -> Result<()> {
        self.shared
            .restored_notify()
            .wait(None)
            .map(|_| ())
            .map_err(Error::new_inner)
    }
}
