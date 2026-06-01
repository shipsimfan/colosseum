use crate::{
    Result,
    render::{RenderData, RenderJob, job::Swapchain},
};
use alexandria::math::Vector2u;

impl<'surface> RenderJob<'surface> {
    /// Run the render job, returning the next state of the job
    pub(crate) fn run(self, window_size: Vector2u, render_data: &RenderData) -> Result<Self> {
        let mut swapchain = match self {
            RenderJob::RecreateSwapchain { device } => {
                return Ok(RenderJob::Rendering {
                    swapchain: Swapchain::new(device, window_size)?,
                });
            }
            RenderJob::Rendering { swapchain } => swapchain,
        };

        if swapchain.next_frame(window_size, render_data)? {
            Ok(RenderJob::RecreateSwapchain {
                device: swapchain.unwrap()?,
            })
        } else {
            Ok(RenderJob::Rendering { swapchain })
        }
    }
}
