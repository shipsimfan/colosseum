use crate::{
    logging::Logger,
    render::RenderData,
    update::{Inputs, Scene, UpdateContext},
};
use alexandria::math::{Color3f, Linear, Vector2u};
use std::time::Duration;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Get the delta time since the last update
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    /// Get the current size of the window
    pub fn window_size(&self) -> Vector2u {
        self.window_size
    }

    /// Create a new logger with the given scope
    pub fn logger(&self, scope: &'static str) -> Logger {
        self.logger.logger(scope)
    }

    /// Get a reference to the settings cache
    pub fn settings(&self) -> &Game::SettingsCache {
        self.settings
    }

    /// Get if the game should exit after this update
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    /// Get the color to clear the screen with before rendering
    pub fn clear_color(&self) -> Color3f<Linear> {
        self.render_data.clear_color()
    }

    /// Get the current set of inputs for the game
    pub fn inputs(&self) -> &Inputs {
        self.inputs
    }

    /// Get the render data for this update
    pub(in crate::update) fn render_data(&mut self) -> &mut RenderData {
        self.render_data
    }

    /// Take the next scene to switch to, if any
    pub(in crate::update) fn take_next_scene(&mut self) -> Option<Box<dyn Scene<Game = Game>>> {
        self.next_scene.take()
    }
}
