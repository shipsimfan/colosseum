use crate::{
    Result,
    render::{
        FrameGraph, FrameGraphTransientBuffer, RenderData, RenderObjects,
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

        transient_buffer: &mut FrameGraphTransientBuffer,

        cmd_buffer: &mut VulkanCommandBuffer,
        memory_properties: &VulkanAdapterMemoryProperties,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Setup the external resources
        let mut resource_builder = FrameGraphResourceBuilder::new(
            &mut self.external_resources,
            &mut self.transient_render_scale_info,
            &mut self.transient_native_scale_info,
            swapchain_size,
            swapchain_image,
            swapchain_color_attachment,
        );

        // See if we need to recompile the frame graph, and do it if needed
        let structure = Some(FrameGraphStructure::from_data(data));
        if self.structure != structure {
            self.structure = structure;
            self.last_swapchain_size = swapchain_size;
            self.last_render_scale = data.render_scale();
            self.transient_epoch += 1;

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
        } else if self.last_swapchain_size != swapchain_size
            || self.last_render_scale != data.render_scale()
        {
            self.last_swapchain_size = swapchain_size;
            self.last_render_scale = data.render_scale();
            self.transient_epoch += 1;
        }

        let (external, transient_render_scale_info, transient_native_scale_info) =
            resource_builder.finish();
        let mut resources = FrameGraphResources::new(external, transient_buffer);

        // See if we need to resize
        if resources.needs_resize(self.transient_epoch) {
            resources.resize(
                &transient_render_scale_info,
                &transient_native_scale_info,
                swapchain_size,
                data.render_scale(),
                memory_properties,
                device,
                self.transient_epoch,
            )?;

            for node in &self.nodes {
                node.update_descriptor_sets(render_objects, &resources, device);
            }
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
