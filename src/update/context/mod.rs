use crate::{logging::Logger, render::RenderData, update::Scene};
use alexandria::math::Vector2u;
use std::time::Duration;

mod get;
mod new;
mod set;

/// The context passed to scenes during the update phase of the game loop
pub struct UpdateContext<'a, Game: crate::Game> {
    /// The delta time since the last update
    delta_time: Duration,

    /// The current size of the window
    window_size: Vector2u,

    /// A logger so the game can create its own loggers
    logger: &'a Logger,

    /// The current settings cache, so the game can read settings
    settings: &'a mut Game::SettingsCache,

    /// The data that will be used for rendering
    render_data: &'a mut RenderData,

    /// Whether the game should exit after this update
    should_exit: bool,

    /// The next scene to switch to, if any
    next_scene: Option<Box<dyn Scene<Game = Game>>>,
}
