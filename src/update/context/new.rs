use crate::{logging::Logger, render::RenderData, update::UpdateContext};
use alexandria::math::Vector2u;
use std::time::Duration;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Create a new update context
    pub(in crate::update) fn new(
        delta_time: Duration,
        window_size: Vector2u,
        logger: &'a Logger,
        settings: &'a mut Game::SettingsCache,
        render_data: &'a mut RenderData,
    ) -> UpdateContext<'a, Game> {
        UpdateContext {
            delta_time,
            window_size,
            logger,
            settings,
            should_exit: false,
            next_scene: None,
            render_data,
        }
    }
}
