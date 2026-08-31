use crate::{
    Result,
    render::{
        FrameGraphTransientBuffer,
        job::{GraphicsDevice, RenderToken},
    },
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanImageView},
    math::Vector2u,
};

impl GraphicsDevice {
    /// Build and run the frame graph to render a frame, returning the pipeline stage flags, access
    /// flags, and image layout for the swapchain image after rendering
    pub fn build_and_run_frame_graph(
        &mut self,
        token: &RenderToken,
        swapchain_size: Vector2u,
        swapchain_color_attachment: &VulkanImageView,
        transient_buffer: &mut FrameGraphTransientBuffer,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) -> Result<()> {
        let render_data = &mut self.render_data[token.frame_index()];

        self.frame_graph.build_and_run(
            render_data,
            &self.render_objects,
            swapchain_size,
            swapchain_color_attachment.image(),
            swapchain_color_attachment,
            transient_buffer,
            cmd_buffer,
            &self.memory_properties,
            &self.device,
        )
    }
}
