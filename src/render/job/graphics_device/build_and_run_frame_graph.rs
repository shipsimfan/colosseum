use crate::render::job::{GraphicsDevice, RenderToken};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl GraphicsDevice {
    /// Build and run the frame graph to render a frame, returning the pipeline stage flags, access
    /// flags, and image layout for the swapchain image after rendering
    pub fn build_and_run_frame_graph(
        &mut self,
        token: RenderToken,
        swapchain_image: &VulkanImageView,
        swapchain_image_size: Vector2u,
        frame_index: usize,
    ) {
        self.frame_graph.build_and_run(
            &self.render_data[token.frame_index()],
            swapchain_image,
            swapchain_image_size,
            &mut self.command_pool[self.command_buffers[frame_index]],
            &self.render_objects,
        )
    }
}
