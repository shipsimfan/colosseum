use crate::{
    Result,
    render::{
        FrameGraph, FrameIndex, GraphicsDevice, SDR_FORMAT, Swapchain,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind, FrameGraphResources,
            SolidColorSkyNode,
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
        frame_index: Res<FrameIndex>,
    ) -> Result<()> {
        if !self.compiled {
            self.compile(&mut resources, commands, &device)?;
        }

        // Handle resizing if needed
        resources.resize(swapchain.size(), &frame_index, &device)?;

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

        let test_resource =
            builder.create_resource("Testing", SDR_FORMAT, FrameGraphResourceKind::Native);

        SolidColorSkyNode::add(test_resource, &mut builder)?;

        builder.finish(resources);

        self.compiled = true;
        Ok(())
    }
}
