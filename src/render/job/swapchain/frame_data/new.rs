use crate::{
    Error, Result,
    render::job::{GraphicsDevice, swapchain::FrameData},
};
use alexandria::gpu::VulkanFenceCreateFlag;

impl FrameData {
    /// Creates a new [`FrameData`]
    pub(in crate::render::job::swapchain) fn new(device: &GraphicsDevice) -> Result<FrameData> {
        let render_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let acquire_image_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let draw_fence = device
            .create_fence(VulkanFenceCreateFlag::Signalled)
            .map_err(Error::new_inner)?;

        Ok(FrameData {
            acquire_image_semaphore,
            render_complete_semaphore,
            draw_fence,
        })
    }
}
