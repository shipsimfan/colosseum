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
        let frame = &mut self.frame_data[token.frame_index()];
        let command_buffer = frame.command_buffer();
        let (render_data, transient_buffer) = frame.render_data_and_transient_buffer_mut();

        self.frame_graph.build_and_run(
            render_data,
            &self.render_objects,
            swapchain_size,
            swapchain_color_attachment.image(),
            swapchain_color_attachment,
            transient_buffer,
            &mut self.command_pool[command_buffer],
            &self.memory_properties,
            &self.device,
        )
    }
}
