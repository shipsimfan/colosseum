use super::Colosseum;
use crate::{logging::LogController, SettingsController};

/// The context which scenes use to update
pub struct UpdateContext<'a> {
    log_controller: &'a LogController,
    settings: &'a mut SettingsController,
}

impl<'a> UpdateContext<'a> {
    /// Creates a new [`UpdateContext`]
    pub(super) fn new(colosseum: &'a mut Colosseum) -> Self {
        UpdateContext {
            log_controller: &colosseum.log_controller,
            settings: &mut colosseum.settings,
        }
    }

    /// Gets the [`LogController`] for this app
    pub fn log_controller(&self) -> &LogController {
        self.log_controller
    }

    /// Gets the [`SettingsController`] for this app
    pub fn settings(&mut self) -> &mut SettingsController {
        self.settings
    }
}
