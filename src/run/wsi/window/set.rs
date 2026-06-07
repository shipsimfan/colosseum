use crate::{Error, Result, UserEvent, Window};

impl Window {
    /// Set the window to fullscreen mode
    pub fn set_fullscreen(&self) -> Result<()> {
        self.event_queue
            .push(UserEvent::SetFullscreen)
            .map_err(Error::new_inner)
    }

    /// Unset the window from fullscreen mode
    pub fn unset_fullscreen(&self) -> Result<()> {
        self.event_queue
            .push(UserEvent::UnsetFullscreen)
            .map_err(Error::new_inner)
    }
}
