use crate::{
    Result,
    render::{
        DescriptorSetLayouts, FrameGraph, FrameIndex, GraphicsDevice, SDR_FORMAT, Samplers,
        Swapchain,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind, FrameGraphResources,
            QuantizationNode, SolidColorSkyNode,
        },
        phases::FrameGraphSetup,
    },
};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanImageAspectFlag};
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
        descriptor_set_layouts: Res<DescriptorSetLayouts>,
        samplers: Res<Samplers>,
    ) -> Result<()> {
        if !self.compiled {
            self.compile(
                &mut resources,
                commands,
                &device,
                &descriptor_set_layouts,
                &samplers,
            )?;
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
        descriptor_set_layouts: &DescriptorSetLayouts,
        samplers: &Samplers,
    ) -> Result<()> {
        let mut builder = FrameGraphBuilder::new(commands, device);

        let main_color_output = builder.create_resource(
            "Main Color Output",
            SDR_FORMAT,
            FrameGraphResourceKind::Native,
            VulkanAttachmentLoadOp::DontCare,
        );

        SolidColorSkyNode::add(main_color_output, &mut builder)?;

        QuantizationNode::add(
            main_color_output,
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            samplers,
            &mut builder,
        )?;

        builder.finish(resources, descriptor_set_layouts)?;

        self.compiled = true;
        Ok(())
    }
}
