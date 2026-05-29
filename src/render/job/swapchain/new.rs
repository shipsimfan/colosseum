use crate::{
    Error, Result,
    render::{
        FrameContext,
        job::{GraphicsDevice, Swapchain},
    },
};
use alexandria::{
    gpu::VulkanSwapchainPresentMode,
    math::{Vector2i, Vector2u},
};

const MAX_FRAMES_IN_FLIGHT: usize = 2;

impl<'surface> Swapchain<'surface> {
    /// Create a new [`Swapchain`] from a [`GraphicsDevice`]
    pub fn new(device: GraphicsDevice<'surface>, size: Vector2u) -> Result<Swapchain<'surface>> {
        let swapchain = device
            .create_swapchain(
                3,
                device.swapchain_format(),
                Vector2i::new(size.x as _, size.y as _),
                VulkanSwapchainPresentMode::Fifo,
                device.surface(),
            )
            .map_err(Error::new_inner)?;

        let image_views = swapchain
            .images()
            .iter()
            .map(|image| {
                image
                    .create_image_view(device.swapchain_format())
                    .map_err(Error::new_inner)
            })
            .collect::<Result<_>>()?;

        let mut frame_contexts = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            frame_contexts.push(FrameContext::new(&device)?);
        }

        Ok(Swapchain {
            device,
            swapchain,
            image_views,
            frame_contexts,
        })
    }
}
