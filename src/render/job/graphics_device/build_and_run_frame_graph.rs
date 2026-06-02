use crate::render::{RenderData, job::GraphicsDevice};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanFormat, VulkanImageView},
    math::Vector2u,
};

impl<'surface> GraphicsDevice<'surface> {
    /// Build and run the frame graph to render a frame
    pub fn build_and_run_frame_graph(
        &mut self,
        data: &RenderData,
        swapchain_image: &VulkanImageView,
        swapchain_image_size: Vector2u,
        swapchain_image_format: VulkanFormat,
        command_buffer: &mut VulkanCommandBuffer,
    ) {
        self.frame_graph.build_and_run(
            data,
            swapchain_image,
            swapchain_image_size,
            swapchain_image_format,
            command_buffer,
            &mut self.frame_graph_resources_pool,
        );
    }
}
