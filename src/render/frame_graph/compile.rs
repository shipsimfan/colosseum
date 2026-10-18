use crate::{
    Result,
    render::{
        FrameGraph, GraphicsDevice, Swapchain,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResources, SolidColorSkyNode,
        },
        phases::FrameGraphSetup,
    },
};
use alexandria::gpu::VulkanImageAspectFlag;
use colosseum_ecs::{Commands, Res, ResMut, World};

impl FrameGraph {
    pub fn register_prepare(world: &mut World) {
        world.register_system(FrameGraphSetup, FrameGraph::prepare);
    }

    /// Prepare the frame graph for execution
    fn prepare(
        mut self: ResMut<Self>,
        mut resources: ResMut<FrameGraphResources>,
        commands: &mut Commands,
        device: Res<GraphicsDevice>,
        swapchain: Res<Swapchain>,
    ) -> Result<()> {
        if !self.compiled {
            self.compile(&mut resources, commands, &device)?;
        }

        // TODO: Handle resizing if needed

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

    /// Compile the frame graph, preparing it for execution
    fn compile(
        &mut self,
        resources: &mut FrameGraphResources,
        commands: &mut Commands,
        device: &GraphicsDevice,
    ) -> Result<()> {
        let mut builder = FrameGraphBuilder::new(commands, device);

        SolidColorSkyNode::add(FrameGraphResourceId::SWAPCHAIN_IMAGE, &mut builder)?;

        builder.finish(resources);

        self.compiled = true;
        Ok(())
    }
}
