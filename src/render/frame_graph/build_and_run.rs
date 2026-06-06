use crate::render::{
    FrameGraph, RenderData,
    frame_graph::{FrameGraphResourceBuilder, FrameGraphStructure},
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanImageView},
    math::Vector2u,
};

impl FrameGraph {
    /// Build, compile, and execute the frame graph
    pub fn build_and_run(
        &mut self,
        data: &RenderData,
        swapchain_image: &VulkanImageView,
        swapchain_image_size: Vector2u,
        command_buffer: &mut VulkanCommandBuffer,
    ) {
        // Setup the external resources
        let mut resource_builder = FrameGraphResourceBuilder::new(
            &mut self.external_resources,
            swapchain_image,
            swapchain_image_size,
        );

        // See if we need to recompile the frame graph, and do it if needed
        let structure = Some(FrameGraphStructure::from_data(data));
        let resources = if self.structure != structure {
            self.structure = structure;

            FrameGraph::build(
                self.structure.as_ref().unwrap(),
                &mut resource_builder,
                &mut self.nodes,
            );

            let mut resources = resource_builder.finish();

            FrameGraph::compile(
                &self.nodes,
                &mut resources,
                &mut self.pipeline_barrier_indices,
                &mut self.pipeline_barriers,
            );

            resources
        } else {
            resource_builder.finish()
        };

        // Execute the frame graph
        FrameGraph::execute(
            data,
            &resources,
            &self.nodes,
            &self.pipeline_barrier_indices,
            &self.pipeline_barriers,
            &mut self.image_barriers,
            &mut self.color_attachments,
            command_buffer,
        );
    }
}
