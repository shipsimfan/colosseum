use crate::{
    Error, Result,
    render::{
        FrameGraphTransientBuffer,
        job::{GraphicsDevice, swapchain::FrameData},
    },
};
use alexandria::gpu::{VulkanCommandBufferLevel, VulkanCommandPool, VulkanFenceCreateFlag};

impl FrameData {
    /// Creates a new [`FrameData`]
    pub(in crate::render::job::swapchain) fn new(
        command_pool: &mut VulkanCommandPool,
        device: &GraphicsDevice,
    ) -> Result<FrameData> {
        let copy_command_buffer = command_pool
            .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
            .map_err(Error::new_inner)?;
        let render_command_buffer = command_pool
            .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
            .map_err(Error::new_inner)?;

        let acquire_image_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let copy_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let render_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;

        let draw_fence = device
            .create_fence(VulkanFenceCreateFlag::Signalled)
            .map_err(Error::new_inner)?;

        let transient_buffer =
            FrameGraphTransientBuffer::new(device.fixed_render_objects(), device)?;

        Ok(FrameData {
            copy_command_buffer,
            render_command_buffer,

            acquire_image_semaphore,
            copy_complete_semaphore,
            render_complete_semaphore,
            draw_fence,

            transient_buffer,
        })
    }
}
