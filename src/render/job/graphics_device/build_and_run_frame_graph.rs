use crate::{
    Result,
    render::job::{GraphicsDevice, RenderToken},
};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl GraphicsDevice {
    /// Build and run the frame graph to render a frame, returning the pipeline stage flags, access
    /// flags, and image layout for the swapchain image after rendering
    pub fn build_and_run_frame_graph(
        &mut self,
        token: &RenderToken,
        swapchain_size: Vector2u,
        swapchain_color_attachment: &VulkanImageView,
    ) -> Result<()> {
        self.frame_graph.build_and_run(
            &self.render_data[token.frame_index()],
            &self.render_objects,
            swapchain_size,
            swapchain_color_attachment.image(),
            swapchain_color_attachment,
            self.swapchain_format,
            &mut self.transient_buffers[token.frame_index()],
            &mut self.command_pool[self.command_buffers[token.frame_index()]],
            &self.memory_properties,
            &self.device,
        )
    }
}
