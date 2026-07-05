use crate::{
    Window,
    file_io::FileIo,
    logging::Logger,
    render::{Material, RenderData, Shader},
    update::{Inputs, UpdateContext},
};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat, VulkanPipelineLayout},
    math::Vector2u,
};
use std::{sync::Arc, time::Duration};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Create a new update context
    pub(in crate::update) fn new(
        delta_time: Duration,
        window_size: Vector2u,
        logger: &'a Logger,
        settings: &'a mut Game::SettingsCache,
        inputs: &'a Inputs,
        file_io: &'a FileIo,
        window: &'a Window,

        render_data: &'a mut RenderData,
        device: &'a VulkanDevice,
        swapchain_format: VulkanFormat,
        pipeline_layout: &'a VulkanPipelineLayout,
        shaders: &'a mut SlotMap<Arc<Shader>>,
        materials: &'a mut SlotMap<Material>,
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
            window,

            render_data,
            device,
            swapchain_format,
            pipeline_layout,
            shaders,
            materials,
        }
    }
}
