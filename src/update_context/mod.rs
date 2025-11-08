use crate::{logging::LogController, run::RunningState};
use std::sync::Arc;

mod get;
mod new;

/// The context used during updates to access the engine
pub struct UpdateContext<'a, Game: crate::Game> {
    /// The amount of time since the last frame, in seconds
    delta_t: f32,

    /// The log controller
    logs: &'a Arc<LogController>,

    /// The settings for the game
    settings: &'a mut Game::SettingsCache,

    /// The running state of the engine
    running_state: &'a RunningState,
}
