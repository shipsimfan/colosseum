use crate::{UpdateContext, logging::LogController, run::RunningState};
use std::sync::Arc;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Create a new [`UpdateContext`]
    pub(crate) fn new(
        delta_t: f32,
        logs: &'a Arc<LogController>,
        settings: &'a mut Game::SettingsCache,
        running_state: &'a RunningState,
    ) -> Self {
        UpdateContext {
            delta_t,
            logs,
            settings,
            running_state,
        }
    }
}
