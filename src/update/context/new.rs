use crate::{
    Window,
    file_io::FileIo,
    logging::Logger,
    render::{RenderData, Skybox},
    update::{ECS, Inputs, UpdateContext, UpdateRenderObjects},
};
use alexandria::math::Vector2u;
use std::time::Duration;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Create a new update context
    pub(in crate::update) fn new(
        delta_time: Duration,
        window_size: Vector2u,
        logger: &'a Logger,
        settings: &'a mut Game::SettingsCache,
        inputs: &'a Inputs,
        file_io: &'a FileIo,
        ecs: &'a mut ECS,
        window: &'a Window,
        skybox: &'a mut Skybox,
        render_data: &'a mut RenderData,
        render_objects: &'a mut UpdateRenderObjects,
    ) -> UpdateContext<'a, Game> {
        UpdateContext {
            delta_time,
            window_size,
            logger,
            settings,
            should_exit: false,
            next_scene: None,
            inputs,
            file_io,
            ecs,
            window,
            skybox,
            render_data,
            render_objects,
        }
    }
}
