use crate::render::{RenderData, job::GraphicsDevice};
use alexandria::{
    gpu::{VulkanAccessFlags, VulkanImageLayout, VulkanImageView, VulkanPipelineStageFlags},
    math::Vector2u,
};

impl GraphicsDevice {
    /// Build and run the frame graph to render a frame, returning the pipeline stage flags, access
    /// flags, and image layout for the swapchain image after rendering
    pub fn build_and_run_frame_graph(
        &mut self,
        data: &RenderData,
        swapchain_image: &VulkanImageView,
        swapchain_image_size: Vector2u,
        frame_index: usize,
    ) -> (
        VulkanPipelineStageFlags,
        VulkanAccessFlags,
        VulkanImageLayout,
    ) {
        self.frame_graph.build_and_run(
            data,
            swapchain_image,
            swapchain_image_size,
            self.swapchain_format,
            &mut self.command_pool[self.command_buffers[frame_index]],
            &mut self.frame_graph_resources_pool,
        )
    }
}
