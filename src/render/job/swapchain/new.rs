use crate::{
    Error, Result, debug,
    render::job::{GraphicsDevice, Swapchain, swapchain::FrameData},
};
use alexandria::{
    gpu::VulkanSwapchainPresentMode,
    math::{Vector2i, Vector2u},
};

const MAX_FRAMES_IN_FLIGHT: usize = 3;

impl<'surface> Swapchain<'surface> {
    /// Create a new [`Swapchain`] from a [`GraphicsDevice`]
    pub fn new(device: GraphicsDevice<'surface>, size: Vector2u) -> Result<Swapchain<'surface>> {
        let swapchain = device
            .create_swapchain(
                MAX_FRAMES_IN_FLIGHT as _,
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
            .collect::<Result<Vec<_>>>()?;

        let mut frame_data = Vec::with_capacity(image_views.len());
        for _ in 0..image_views.len() {
            frame_data.push(FrameData::new(&device)?);
        }

        debug!(
            device.logger(),
            "Created swapchain sized {}x{}", size.x, size.y
        );

        Ok(Swapchain {
            device,
            swapchain,
            image_views,
            frame_data,
            frame_index: 0,
            size,
        })
    }
}
