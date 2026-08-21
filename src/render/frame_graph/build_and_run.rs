use crate::{
    Result,
    render::{
        FrameGraph, RenderData, RenderObjects,
        frame_graph::{
            FrameGraphResourceBuilder, FrameGraphResourceId, FrameGraphResources,
            FrameGraphStructure,
        },
    },
};
use alexandria::{
    gpu::{
        VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanDevice, VulkanImage,
        VulkanImageView,
    },
    math::Vector2u,
};

impl FrameGraph {
    /// Build, compile, and execute the frame graph
    pub fn build_and_run(
        &mut self,
        data: &RenderData,
        render_objects: &RenderObjects,

        swapchain_size: Vector2u,
        swapchain_image: &VulkanImage,
        swapchain_color_attachment: &VulkanImageView,

        cmd_buffer: &mut VulkanCommandBuffer,
        memory_properties: &VulkanAdapterMemoryProperties,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Setup the external resources
        let mut resource_builder = FrameGraphResourceBuilder::new(
            &mut self.external_resources,
            &mut self.transient_render_scale_info,
            swapchain_size,
            swapchain_image,
            swapchain_color_attachment,
        );

        // See if we need to recompile the frame graph, and do it if needed
        let structure = Some(FrameGraphStructure::from_data(data));
        if self.structure != structure {
            self.structure = structure;
            self.last_swapchain_size = Vector2u::ZERO;

            FrameGraph::build(
                self.structure.as_ref().unwrap(),
                &mut resource_builder,
                &mut self.nodes,
            );

            FrameGraph::compile(
                &self.nodes,
                &mut resource_builder,
                &mut self.pipeline_barrier_indices,
                &mut self.pipeline_barriers,
            );

            self.swapchain_final_state = resource_builder
                .get_external(FrameGraphResourceId::SWAPCHAIN_IMAGE)
                .state()
                .clone();
        };
        let (external, transient_render_scale_info) = resource_builder.finish();
        let mut resources = FrameGraphResources::new(
            external,
            &mut self.transient_render_scale,
            &mut self.transient_render_scale_memory,
        );

        // See if we need to resize
        if self.last_swapchain_size != swapchain_size {
            self.last_swapchain_size = swapchain_size;
            resources.resize(
                &transient_render_scale_info,
                swapchain_size,
                1.0,
                memory_properties,
                device,
            )?;
        }

        // Execute the frame graph
        FrameGraph::execute(
            data,
            &resources,
            &self.nodes,
            &self.pipeline_barrier_indices,
            &self.pipeline_barriers,
            &mut self.image_barriers,
            &mut self.color_attachments,
            cmd_buffer,
            self.swapchain_final_state.clone(),
            render_objects,
        );

        Ok(())
    }
}
