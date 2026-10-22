use crate::{
    Result,
    render::{FrameIndex, GraphicsDevice, frame_graph::FrameGraphResources},
};
use alexandria::math::Vector2u;

impl FrameGraphResources {
    /// Resize the frame graph resources to match the new swapchain size
    pub fn resize(
        &mut self,
        swapchain_size: Vector2u,
        frame_index: &FrameIndex,
        device: &GraphicsDevice,
    ) -> Result<()> {
        let frame_index = **frame_index;

        self.native_scale
            .resize(swapchain_size, frame_index, device, &self.descriptor_sets)?;

        Ok(())
    }
}
