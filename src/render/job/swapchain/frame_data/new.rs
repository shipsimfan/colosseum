use crate::{
    Error, Result,
    render::job::{GraphicsDevice, swapchain::FrameData},
};

impl FrameData {
    /// Creates a new [`FrameData`]
    pub(in crate::render::job::swapchain) fn new(device: &GraphicsDevice) -> Result<FrameData> {
        let command_buffer = device.allocate_command_buffer()?;
        let render_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let present_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let draw_fence = device.create_fence(true).map_err(Error::new_inner)?;

        Ok(FrameData {
            command_buffer,
            render_complete_semaphore,
            present_complete_semaphore,
            draw_fence,
        })
    }
}
