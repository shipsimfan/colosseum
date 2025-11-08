use crate::{UpdateContext, logging::LogController};
use std::sync::Arc;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Get the amount of time that has passed since the last frame, in seconds
    pub fn delta_t(&self) -> f32 {
        self.delta_t
    }

    /// Get the log controller to create loggers
    pub fn logs(&self) -> &Arc<LogController> {
        self.logs
    }

    /// Get the settings for the game
    pub fn settings(&mut self) -> &mut Game::SettingsCache {
        self.settings
    }

    /// Quit the game, exiting the application
    pub fn quit(&self) {
        self.running_state.kill();
    }
}
