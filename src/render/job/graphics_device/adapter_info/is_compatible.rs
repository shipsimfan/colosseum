use crate::{
    Error, Result, info,
    logging::Logger,
    render::job::{Swapchain, graphics_device::VulkanAdapterInfo},
    warning,
};
use alexandria::gpu::{
    VulkanAdapter, VulkanDeviceFeatures, VulkanDeviceVulkan13Features, VulkanFormat, VulkanSurface,
};

impl<'instance> VulkanAdapterInfo<'instance> {
    /// Is adapter compatible with the surface and suitable for rendering?
    pub fn is_compatible_adapter(
        adapter: VulkanAdapter<'instance>,
        surface: &VulkanSurface,
        logger: Option<&Logger>,
    ) -> Result<Option<VulkanAdapterInfo<'instance>>> {
        if let Some(logger) = logger {
            info!(
                logger,
                "Found adapter: {} ({})",
                adapter.name(),
                adapter.uuid()
            );
        }

        // Determine if the adapter has the supported features
        let mut vulkan_13_features = VulkanDeviceVulkan13Features::default();
        adapter.get_features([
            &mut VulkanDeviceFeatures::default() as _,
            &mut vulkan_13_features as _,
        ]);

        if !vulkan_13_features.synchronization2() || !vulkan_13_features.dynamic_rendering() {
            if let Some(logger) = logger {
                warning!(
                    logger,
                    "Adapter \"{}\" rejected because it does not support required Vulkan 1.3 features",
                    adapter.name()
                );
            }
            return Ok(None);
        }

        // Determine if the adapter supports a compatible swapchain format
        let swapchain_format = adapter
            .swapchain_formats(surface)
            .map_err(Error::new_inner)?
            .into_iter()
            .filter_map(|format| {
                match format.color_space {
                    Swapchain::COLOR_SPACE => (),
                    _ => return None,
                }

                match format.format {
                    VulkanFormat::B8G8R8A8UNorm | VulkanFormat::R8G8B8A8UNorm => {
                        Some(format.format)
                    }
                    _ => None,
                }
            })
            .next();

        let swapchain_format = match swapchain_format {
            Some(swapchain_format) => swapchain_format,
            None => {
                if let Some(logger) = logger {
                    warning!(
                        logger,
                        "Adapter \"{}\" rejected because it does not support a compatible swapchain format",
                        adapter.name()
                    );
                }
                return Ok(None);
            }
        };

        // Find the best graphics queue family
        let mut graphics_queue_family_index = None;
        for (index, queue_family) in adapter.queue_families().iter().enumerate() {
            // Check if the queue family supports graphics operations
            if !queue_family.graphics() {
                continue;
            }

            // Determine if the queue family supports the surface
            let index = index as u32;
            if !adapter
                .supports_surface(index, surface)
                .map_err(Error::new_inner)?
            {
                continue;
            }

            graphics_queue_family_index = Some(index);
            break;
        }

        Ok(match graphics_queue_family_index {
            Some(graphics_queue_family_index) => Some(VulkanAdapterInfo {
                adapter,
                swapchain_format,
                graphics_queue_family_index,
            }),
            None => {
                if let Some(logger) = logger {
                    warning!(
                        logger,
                        "Adapter \"{}\" rejected because it does not have a compatible graphics queue family",
                        adapter.name()
                    );
                }
                None
            }
        })
    }
}
