use crate::{
    Error, Result, debug,
    render::job::{GraphicsDevice, Swapchain, swapchain::FrameData},
};
use alexandria::{
    gpu::{
        VulkanCommandPoolCreateFlag, VulkanComponentMapping, VulkanCompositeAlphaFlag,
        VulkanImageAspectFlag, VulkanImageUsageFlag, VulkanImageViewType, VulkanPresentMode,
        VulkanSharingMode, VulkanSurface,
    },
    math::Vector2u,
};
#[cfg(debug_assertions)]
use std::ffi::CString;

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

        let mut image_views = Vec::with_capacity(swapchain.images().len());
        for (i, image) in swapchain.images().iter().enumerate() {
            let mut image_view = image
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
                .map_err(Error::new_inner)?;

            #[cfg(debug_assertions)]
            {
                let image_view_name = CString::new(format!("Swapchain Image View {}", i)).unwrap();
                device
                    .set_object_name(&mut image_view, &image_view_name)
                    .map_err(Error::new_inner)?;
            }

            image_views.push(image_view)
        }

        // Create the command pool
        let render_queue_family = device.render_queue_family();
        let mut command_pool = device
            .create_command_pool(
                render_queue_family,
                VulkanCommandPoolCreateFlag::ResetCommandBuffer,
            )
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut command_pool, c"Swapchain Command Pool")
            .map_err(Error::new_inner)?;

        // Allocate per-frame data
        let shadow_quality = device.render_data().lighting().shadow_quality();
        let mut frame_data = Vec::with_capacity(image_views.len());
        for i in 0..image_views.len() {
            frame_data.push(FrameData::new(
                i,
                &mut command_pool,
                &device,
                shadow_quality,
            )?);
        }

        device.reserve_render_data(image_views.len())?;

        debug!(
            device.logger(),
            "Created swapchain sized {}x{}", size.x, size.y
        );

        Ok(Swapchain {
            swapchain: Some(swapchain),
            image_views,
            command_pool,
            frame_data,
            frame_index: 0,
            size,
            device: device.clone(),
        })
    }
}
