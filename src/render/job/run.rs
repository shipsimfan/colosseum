use crate::{
    Error, Result,
    render::{RenderJob, job::Swapchain},
};
use alexandria::math::Vector2u;

impl<'surface> RenderJob<'surface> {
    /// Run the render job, returning the next state of the job
    pub(crate) fn run(self, window_size: Vector2u) -> Result<Self> {
        match self {
            RenderJob::RecreateSwapchain { device } => {
                device.wait_idle().map_err(Error::new_inner)?;
                Ok(RenderJob::Rendering {
                    swapchain: Swapchain::new(device, window_size)?,
                })
            }
            RenderJob::Rendering { swapchain } => {
                std::thread::sleep(std::time::Duration::from_millis(16));
                Ok(RenderJob::Rendering { swapchain })
            }
        }
    }
}
