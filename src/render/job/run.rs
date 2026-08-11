use crate::{
    Result,
    render::{RenderJob, job::Swapchain},
};
use alexandria::math::Vector2u;

impl<'surface> RenderJob<'surface> {
    /// Run the render job, returning the next state of the job
    pub(crate) fn run(self, window_size: Vector2u) -> Result<RenderJob<'surface>> {
        Ok(match self {
            RenderJob::RecreateSwapchain {
                mut device,
                surface,
            } => RenderJob::Rendering {
                swapchain: Swapchain::new(surface, window_size, &mut device)?,
                device,
            },
            RenderJob::Rendering {
                mut device,
                mut swapchain,
            } => {
                if swapchain.next_frame(window_size, &mut device)? {
                    RenderJob::RecreateSwapchain {
                        surface: swapchain.unwrap(&device)?,
                        device,
                    }
                } else {
                    RenderJob::Rendering { device, swapchain }
                }
            }
        })
    }
}
