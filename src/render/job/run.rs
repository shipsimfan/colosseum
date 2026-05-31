use crate::{
    Result,
    render::{RenderJob, job::Swapchain},
};
use alexandria::math::{Color4f, Srgb, Vector2u};

impl<'surface> RenderJob<'surface> {
    /// Run the render job, returning the next state of the job
    pub(crate) fn run(self, window_size: Vector2u) -> Result<Self> {
        match self {
            RenderJob::RecreateSwapchain { device } => Ok(RenderJob::Rendering {
                swapchain: Swapchain::new(device, window_size)?,
            }),
            RenderJob::Rendering { swapchain } => render_frame(swapchain, window_size),
        }
    }
}

fn render_frame<'surface>(
    mut swapchain: Swapchain<'surface>,
    size: Vector2u,
) -> Result<RenderJob<'surface>> {
    let mut frame = match swapchain.next_frame(size)? {
        Some(frame) => frame,
        None => {
            return Ok(RenderJob::RecreateSwapchain {
                device: swapchain.unwrap()?,
            });
        }
    };

    let clear_color = Color4f::<Srgb>::new(1.0, 0.0, 1.0, 1.0);
    frame.begin_rendering_swapchain(clear_color);
    frame.cmd_end_rendering();

    frame.present()?;
    Ok(RenderJob::Rendering { swapchain })
}
