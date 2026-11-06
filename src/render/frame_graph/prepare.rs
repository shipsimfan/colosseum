use crate::{
    Result, SettingsCache,
    render::{
        DescriptorSetLayouts, FrameGraph, FrameIndex, GpuDataBuffers, GpuDirectionalLight,
        GpuPointLight, GpuSpotLight, HostDataBuffer, Samplers, Skybox, Swapchain,
        frame_graph::{FrameGraphResourceId, FrameGraphResources},
        phases::FrameGraphSetup,
    },
};
use alexandria::gpu::VulkanImageAspectFlag;
use colosseum_ecs::{Commands, Res, ResMut, World};

impl<Game: crate::Game> FrameGraph<Game> {
    pub fn register_prepare(world: &mut World) {
        world.register_system(FrameGraphSetup, FrameGraph::<Game>::prepare);
    }

    /// Prepare the frame graph for execution
    fn prepare(
        mut self: ResMut<Self>,
        mut resources: ResMut<FrameGraphResources>,
        commands: &mut Commands,

        swapchain: Res<Swapchain>,
        frame_index: Res<FrameIndex>,
        descriptor_set_layouts: Res<DescriptorSetLayouts>,
        data_buffers: Res<GpuDataBuffers>,
        samplers: Res<Samplers>,

        directional_lights: Res<HostDataBuffer<GpuDirectionalLight>>,
        point_lights: Res<HostDataBuffer<GpuPointLight>>,
        spot_lights: Res<HostDataBuffer<GpuSpotLight>>,

        settings: Res<Game::SettingsCache>,
        skybox: Res<Skybox>,
    ) -> Result<()> {
        // Compile if needed
        self.compile(
            &mut resources,
            commands,
            &data_buffers,
            &descriptor_set_layouts,
            &samplers,
            &settings,
            &skybox,
        )?;

        // Handle resizing if needed
        let display_settings = settings.display_settings();
        resources.resize(
            swapchain.size(),
            display_settings.render_scale(),
            &data_buffers,
            display_settings.shadow_quality(),
            &directional_lights,
            &point_lights,
            &spot_lights,
            &frame_index,
            &self.device,
        )?;

        // Update swapchain image in frame graph resources
        resources.insert_external(
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            swapchain.image_view(),
            swapchain.size(),
            1,
            VulkanImageAspectFlag::Color,
        );

        Ok(())
    }
}
