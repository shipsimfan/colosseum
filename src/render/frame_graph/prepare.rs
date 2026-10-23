use crate::{
    Result, SettingsCache,
    render::{
        DescriptorSetLayouts, FrameGraph, FrameIndex, Samplers, Swapchain,
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
        settings: Res<Game::SettingsCache>,
        swapchain: Res<Swapchain>,
        frame_index: Res<FrameIndex>,
        descriptor_set_layouts: Res<DescriptorSetLayouts>,
        samplers: Res<Samplers>,
    ) -> Result<()> {
        // Compile if needed
        self.compile(
            &mut resources,
            commands,
            &descriptor_set_layouts,
            &samplers,
            &settings,
        )?;

        // Handle resizing if needed
        resources.resize(
            swapchain.size(),
            settings.display_settings().render_scale(),
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
