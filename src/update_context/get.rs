use crate::{Scene, UpdateContext, graphics::GraphicsContext, logging::LogController};
use std::sync::Arc;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Get the amount of time that has passed since the last frame, in seconds
    pub const fn delta_t(&self) -> f32 {
        self.delta_t
    }

    /// Get the log controller to create loggers
    pub const fn logs(&self) -> &Arc<LogController> {
        self.logs
    }

    /// Get the input from the player
    pub const fn input(&self) -> &Game::Input {
        self.input
    }

    /// Get the settings for the game
    pub fn settings(&mut self) -> &mut Game::SettingsCache {
        self.settings
    }

    /// Get the graphics context for creating renderables
    pub fn graphics(&mut self) -> &mut GraphicsContext {
        self.graphics_context
    }

    /// Take the next scene from this context, if one was set
    pub(crate) fn take_next_scene(&mut self) -> Option<Box<dyn Scene<Game = Game>>> {
        self.next_scene.take()
    }
}
