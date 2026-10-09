use crate::{Result, UserEvent, Window};
use colosseum_core::new_error;

impl Window {
    /// Set the window to fullscreen mode
    pub fn set_fullscreen(&self) -> Result<()> {
        self.event_queue
            .push(UserEvent::SetFullscreen)
            .map_err(|error| new_error!("unable to set window to fullscreen - {}", error))
    }

    /// Unset the window from fullscreen mode
    pub fn unset_fullscreen(&self) -> Result<()> {
        self.event_queue
            .push(UserEvent::UnsetFullscreen)
            .map_err(|error| new_error!("unable to unset window from fullscreen - {}", error))
    }
}
