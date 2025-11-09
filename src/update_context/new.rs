use crate::{UpdateContext, graphics::GraphicsContext, logging::LogController, run::RunningState};
use std::sync::Arc;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Create a new [`UpdateContext`]
    pub(crate) fn new(
        delta_t: f32,
        logs: &'a Arc<LogController>,
        input: &'a Game::Input,
        settings: &'a mut Game::SettingsCache,
        graphics_context: &'a mut GraphicsContext,
        running_state: &'a RunningState,
    ) -> Self {
        UpdateContext {
            delta_t,
            logs,
            input,
            settings,
            graphics_context,
            running_state,
        }
    }
}
