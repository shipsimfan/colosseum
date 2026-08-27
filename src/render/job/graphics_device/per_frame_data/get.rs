use crate::render::{FrameGraphTransientBuffer, RenderData, job::graphics_device::PerFrameData};
use alexandria::{Id, gpu::VulkanCommandBuffer};

impl PerFrameData {
    /// Get the ID of the command buffer for this frame
    pub fn command_buffer(&self) -> Id<VulkanCommandBuffer> {
        self.command_buffer
    }

    /// Get a reference to the render data and a mutable reference to the transient buffer for this
    /// frame
    pub fn render_data_and_transient_buffer_mut(
        &mut self,
    ) -> (&RenderData, &mut FrameGraphTransientBuffer) {
        (&self.render_data, &mut self.transient_buffer)
    }

    /// Get a mutable reference to the render data for this frame
    pub fn render_data_mut(&mut self) -> &mut RenderData {
        &mut self.render_data
    }
}
