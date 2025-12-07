use crate::{
    ManagedObjects, Scene, graphics::GraphicsContext, logging::LogController, run::RunningState,
};
use std::sync::Arc;

mod deref;
mod get;
mod new;
mod quit;
mod set;

/// A function which creates a new scene for a game
pub(crate) type NextSceneFn<Game> =
    Box<dyn FnOnce(&mut UpdateContext<Game>) -> Box<dyn Scene<Game = Game>>>;

/// The context used during updates to access the engine
pub struct UpdateContext<'a, Game: crate::Game> {
    /// The log controller
    pub logs: &'a Arc<LogController>,

    /// The settings for the game
    pub settings: &'a mut Game::SettingsCache,

    /// The graphics context for creating rendering objects
    pub graphics_context: &'a mut GraphicsContext,

    /// The input tracking for the game
    pub input: &'a Game::Input,

    /// The set of objects managed by the engine
    pub objects: &'a mut ManagedObjects,

    /// The amount of time since the last frame, in seconds
    delta_t: f32,

    /// The running state of the engine
    running_state: &'a RunningState,

    /// The next scene to run on the game
    next_scene: Option<NextSceneFn<Game>>,
}
