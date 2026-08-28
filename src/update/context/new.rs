use crate::{
    Window,
    file_io::FileIo,
    logging::Logger,
    render::{RenderData, Skybox},
    update::{ECS, Entity, Inputs, UpdateContext, UpdateRenderObjects},
};
use alexandria::{Id, math::Vector2u};
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
        active_camera: &'a mut Option<Id<Entity>>,
        window: &'a Window,
        skybox: &'a mut Skybox,
        exposure: &'a mut f32,
        contrast: &'a mut f32,
        saturation: &'a mut f32,
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
            active_camera,
            window,
            skybox,
            exposure,
            contrast,
            saturation,
            render_data,
            render_objects,
        }
    }
}
