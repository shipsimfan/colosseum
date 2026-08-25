use crate::{
    Error, Result, debug,
    render::job::{GraphicsDevice, Swapchain, swapchain::FrameData},
};
use alexandria::{
    gpu::{
        VulkanComponentMapping, VulkanCompositeAlphaFlag, VulkanImageAspectFlag,
        VulkanImageUsageFlag, VulkanImageViewType, VulkanPresentMode, VulkanSharingMode,
        VulkanSurface,
    },
    math::Vector2u,
};

const MIN_FRAMES_IN_FLIGHT: usize = 2;

impl<'surface> Swapchain<'surface> {
    /// Create a new [`Swapchain`] from a [`GraphicsDevice`]
    pub fn new(
        surface: &'surface mut VulkanSurface,
        size: Vector2u,
        device: &mut GraphicsDevice,
    ) -> Result<Swapchain<'surface>> {
        let capabilities = device
            .surface_capabilities(surface)
            .map_err(Error::new_inner)?;
        let size = capabilities
            .current_extent()
            .unwrap_or(capabilities.clamp_image_extent(size));

        let swapchain_format = device.swapchain_format();
        let swapchain = device
            .create_swapchain(
                0,
                surface,
                MIN_FRAMES_IN_FLIGHT as _,
                swapchain_format,
                Swapchain::COLOR_SPACE,
                size,
                1,
                VulkanImageUsageFlag::ColorAttachment | VulkanImageUsageFlag::TransferDst,
                VulkanSharingMode::Exclusive,
                &[],
                capabilities.current_transform(),
                VulkanCompositeAlphaFlag::OpaqueKhr,
                VulkanPresentMode::FIFOKhr,
                true,
                None,
            )
            .map_err(Error::new_inner)?;

        let image_views = swapchain
            .images()
            .iter()
            .map(|image| {
                image
                    .create_image_view(
                        0,
                        VulkanImageViewType::_2d,
                        device.swapchain_format(),
                        VulkanComponentMapping::default(),
                        VulkanImageAspectFlag::Color,
                        0,
                        1,
                        0,
                        1,
                    )
                    .map_err(Error::new_inner)
            })
            .collect::<Result<Vec<_>>>()?;

        let mut frame_data = Vec::with_capacity(image_views.len());
        for _ in 0..image_views.len() {
            frame_data.push(FrameData::new(&device)?);
        }

        device.reserve_command_buffers(image_views.len())?;

        debug!(
            device.logger(),
            "Created swapchain sized {}x{}", size.x, size.y
        );

        Ok(Swapchain {
            swapchain,
            image_views,
            frame_data,
            frame_index: 0,
            size,
        })
    }
}
