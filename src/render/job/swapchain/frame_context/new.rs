use crate::{
    Error, Result,
    render::{FrameContext, job::GraphicsDevice},
};

impl FrameContext {
    /// Creates a new [`FrameContext`]
    pub(in crate::render::job::swapchain) fn new(device: &GraphicsDevice) -> Result<FrameContext> {
        let command_buffer = device.allocate_command_buffer()?;
        let render_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let present_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let draw_fence = device.create_fence(true).map_err(Error::new_inner)?;

        Ok(FrameContext {
            command_buffer,
            render_complete_semaphore,
            present_complete_semaphore,
            draw_fence,
        })
    }
}
