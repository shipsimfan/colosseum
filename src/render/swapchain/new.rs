use crate::{
    Result, debug,
    render::{GraphicsDevice, MAX_FRAMES_IN_FLIGHT, Swapchain},
};
use alexandria::{
    gpu::{
        VulkanComponentMapping, VulkanCompositeAlphaFlag, VulkanImageAspectFlag,
        VulkanImageUsageFlag, VulkanImageViewType, VulkanPresentMode, VulkanSharingMode,
        VulkanSurface,
    },
    math::Vector2u,
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl Swapchain {
    /// Create a new [`Swapchain`] from a [`GraphicsDevice`]
    pub fn new(
        surface: &mut VulkanSurface,
        size: Vector2u,
        device: &mut GraphicsDevice,
    ) -> Result<Swapchain> {
        let capabilities = device
            .surface_capabilities(surface)
            .map_err(|error| new_error!("unable to get surface capabilities - {}", error))?;
        let size = capabilities
            .current_extent()
            .unwrap_or(capabilities.clamp_image_extent(size));

        let swapchain_format = device.swapchain_format();
        let swapchain = device
            .create_swapchain(
                0,
                surface,
                MAX_FRAMES_IN_FLIGHT as _,
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
            .map_err(|error| new_error!("unable to create swapchain - {}", error))?;

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
                .map_err(|error| new_error!("unable to create swapchain image view - {}", error))?;

            #[cfg(debug_assertions)]
            {
                let image_view_name = CString::new(format!("Swapchain Image View {}", i)).unwrap();
                device
                    .set_object_name(&mut image_view, &image_view_name)
                    .map_err(|error| {
                        new_error!("unable to set swapchain image view name - {}", error)
                    })?;
            }

            image_views.push(image_view)
        }

        debug!(
            device.logger(),
            "Created swapchain sized {}x{}", size.x, size.y
        );

        Ok(Swapchain {
            logger: device.logger().clone(),
            swapchain,
            image_views,
            size,
            device: device.clone(),
            image_index: 0,
            valid: true,
            suboptimal: false,
        })
    }
}
