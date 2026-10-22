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
        render_scale: f32,
        frame_index: &FrameIndex,
        device: &GraphicsDevice,
    ) -> Result<()> {
        let frame_index = **frame_index;

        self.native_scale
            .resize(swapchain_size, frame_index, device, &self.descriptor_sets)?;

        let render_scale_size = (swapchain_size.into_f32() * render_scale).round();
        self.render_scale.resize(
            (render_scale_size.x as u32, render_scale_size.y as u32).into(),
            frame_index,
            device,
            &self.descriptor_sets,
        )?;

        Ok(())
    }
}
