use crate::render::{FrameGraph, RenderData, frame_graph::FrameGraphResourcesPool};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanFormat, VulkanImageView},
    math::Vector2u,
};

impl FrameGraph {
    /// Build, compile, and execute the frame graph
    pub fn build_and_run(
        &mut self,
        data: &RenderData,
        swapchain_image: &VulkanImageView,
        swapchain_image_size: Vector2u,
        swapchain_image_format: VulkanFormat,
        command_buffer: &mut VulkanCommandBuffer,
        resource_pool: &mut FrameGraphResourcesPool,
    ) {
        // Reset the frame graph
        self.nodes.clear();
        let mut resources = resource_pool.begin();

        // Build the frame graph for this frame
        self.build(
            data,
            swapchain_image,
            swapchain_image_size,
            swapchain_image_format,
            &mut resources,
        );
        self.compile();
        self.execute(data, command_buffer, &resources);
    }
}
