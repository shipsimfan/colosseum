use super::Colosseum;
use crate::{logging::LogController, SettingsController};

/// The context which scenes use to update
pub struct UpdateContext<'a>(&'a mut Colosseum);

impl<'a> UpdateContext<'a> {
    /// Creates a new [`UpdateContext`]
    pub(super) fn new(colosseum: &'a mut Colosseum) -> Self {
        UpdateContext(colosseum)
    }

    /// Gets the [`LogController`] for this app
    pub fn log_controller(&self) -> &LogController {
        &self.0.log_controller
    }

    /// Gets the [`SettingsController`] for this app
    pub fn settings(&mut self) -> &mut SettingsController {
        &mut self.0.settings
    }

    /// Signals the application to close at the end of the frame
    pub fn exit(&mut self) {
        self.0.graphics.window().exit()
    }
}
