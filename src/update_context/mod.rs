use crate::{graphics::GraphicsContext, logging::LogController, run::RunningState};
use std::sync::Arc;

mod get;
mod new;
mod quit;

/// The context used during updates to access the engine
pub struct UpdateContext<'a, Game: crate::Game> {
    /// The amount of time since the last frame, in seconds
    delta_t: f32,

    /// The log controller
    logs: &'a Arc<LogController>,

    /// The settings for the game
    settings: &'a mut Game::SettingsCache,

    /// The graphics context for creating rendering objects
    graphics_context: &'a mut GraphicsContext,

    /// The input tracking for the game
    input: &'a Game::Input,

    /// The running state of the engine
    running_state: &'a RunningState,
}
