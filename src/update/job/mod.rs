use crate::{logging::Logger, update::Scene};

mod new;
mod run;

/// The persistent state of the update job
pub(crate) struct UpdateJob<'a, Game: crate::Game> {
    /// The current scene being updated
    scene: Box<dyn Scene<Game = Game>>,

    /// The scene to transition to at the start of the next frame, if any
    next_scene: Option<Box<dyn Scene<Game = Game>>>,

    /// A logger to use during the update phase
    logger: Logger,

    /// The game's settings cache
    settings: &'a mut Game::SettingsCache,
}
