use crate::{
    Error, Result, info,
    logging::Logger,
    render::job::{Swapchain, graphics_device::VulkanAdapterInfo},
    warning,
};
use alexandria::{
    MemorySize,
    gpu::{
        VulkanAdapter, VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures,
        VulkanDeviceVulkan11Features, VulkanDeviceVulkan13Features, VulkanFormat, VulkanSurface,
    },
};
use std::sync::Arc;

impl<'instance> VulkanAdapterInfo<'instance> {
    /// Is adapter compatible with the surface and suitable for rendering?
    pub fn is_compatible_adapter(
        adapter: VulkanAdapter<'instance>,
        surface: &VulkanSurface,
        logger: &Logger,
    ) -> Result<Option<VulkanAdapterInfo<'instance>>> {
        let properties = adapter.get_properties();
        let name = properties.device_name().into_owned();

        info!(
            logger,
            "Found adapter: {} ({})",
            name,
            properties.pipeline_cache_uuid()
        );

        // Determine if the adapter has the supported features
        if !has_required_features(&adapter, &name, logger) {
            return Ok(None);
        }

        // Determine if the adapter supports a compatible swapchain format
        let swapchain_format = match find_swapchain_format(&adapter, surface, &name, logger)? {
            Some(format) => format,
            None => return Ok(None),
        };

        // Find the graphics and transfer queue families
        let (graphics_queue_family_index, transfer_queue_family_index) =
            match find_queue_family_indices(&adapter, surface, &properties.device_name(), logger)? {
                Some(indices) => indices,
                None => return Ok(None),
            };

        // Get the memory properties of the adapter and the total amount of device-local VRAM available
        let memory_properties = Arc::new(adapter.get_memory_properties());
        let mut device_local_vram = 0;
        for memory_type in memory_properties.memory_types() {
            if memory_type.device_local() {
                device_local_vram +=
                    *memory_properties.memory_heaps()[memory_type.heap_index()].size();
            }
        }
        let device_local_vram = MemorySize::new(device_local_vram);

        Ok(Some(VulkanAdapterInfo {
            adapter,
            name,
            uuid: properties.pipeline_cache_uuid(),
            r#type: properties.device_type(),
            swapchain_format,
            graphics_queue_family_index,
            transfer_queue_family_index,
            device_local_vram,
            memory_properties,
        }))
    }
}

/// Check if the given adapter has the required features
fn has_required_features(adapter: &VulkanAdapter, device_name: &str, logger: &Logger) -> bool {
    let mut vulkan_11_features = VulkanDeviceVulkan11Features::default();
    let mut vulkan_13_features = VulkanDeviceVulkan13Features::default();
    let mut extended_dynamic_state = VulkanDeviceExtendedDynamicStateFeatures::default();
    adapter.get_features([
        &mut VulkanDeviceFeatures::default() as _,
        &mut vulkan_11_features as _,
        &mut vulkan_13_features as _,
        &mut extended_dynamic_state as _,
    ]);

    if !vulkan_11_features.shader_draw_parameters() {
        warning!(
            logger,
            "Adapter \"{}\" rejected because it does not support required Vulkan 1.1 features",
            device_name,
        );
        return false;
    }

    if !vulkan_13_features.synchronization2() || !vulkan_13_features.dynamic_rendering() {
        warning!(
            logger,
            "Adapter \"{}\" rejected because it does not support required Vulkan 1.3 features",
            device_name,
        );
        return false;
    }

    if !extended_dynamic_state.extended_dynamic_state() {
        warning!(
            logger,
            "Adapter \"{}\" rejected because it does not support required Vulkan extended dynamic state features",
            device_name
        );
        return false;
    }

    true
}

/// Determine if the given adapter is compatible with the surface and suitable for rendering
fn find_swapchain_format(
    adapter: &VulkanAdapter,
    surface: &VulkanSurface,
    device_name: &str,
    logger: &Logger,
) -> Result<Option<VulkanFormat>> {
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
                VulkanFormat::B8G8R8A8UNorm | VulkanFormat::R8G8B8A8UNorm => Some(format.format),
                _ => None,
            }
        })
        .next();

    if swapchain_format.is_none() {
        warning!(
            logger,
            "Adapter \"{}\" rejected because it does not support a compatible swapchain format",
            device_name
        );
    }

    Ok(swapchain_format)
}

/// Find the graphics and transfer queue family indices
fn find_queue_family_indices(
    adapter: &VulkanAdapter,
    surface: &VulkanSurface,
    device_name: &str,
    logger: &Logger,
) -> Result<Option<(u32, u32)>> {
    let mut graphics_queue_family_index = None;
    let mut transfer_queue_family_index = None;
    for (index, queue_family) in adapter
        .get_queue_family_properties()
        .into_iter()
        .enumerate()
    {
        // Check if the queue family supports graphics operations
        if graphics_queue_family_index.is_none() && queue_family.graphics() {
            // Determine if the queue family supports the surface
            let index = index as u32;
            if !adapter
                .supports_surface(index, surface)
                .map_err(Error::new_inner)?
            {
                continue;
            }

            graphics_queue_family_index = Some(index);
        } else if transfer_queue_family_index.is_none()
            && queue_family.transfer()
            && !(queue_family.graphics() || queue_family.compute())
        {
            transfer_queue_family_index = Some(index as u32);
        }

        if graphics_queue_family_index.is_some() && transfer_queue_family_index.is_some() {
            break;
        }
    }

    let graphics_queue_family_index = match graphics_queue_family_index {
        Some(graphics_queue_family_index) => graphics_queue_family_index,
        None => {
            warning!(
                logger,
                "Adapter \"{}\" rejected because it does not have a compatible graphics queue family",
                device_name,
            );
            return Ok(None);
        }
    };
    let transfer_queue_family_index = match transfer_queue_family_index {
        Some(transfer_queue_family_index) => transfer_queue_family_index,
        None => {
            warning!(
                logger,
                "Adapter \"{}\" rejected because it does not have a compatible transfer queue family",
                device_name,
            );
            return Ok(None);
        }
    };

    Ok(Some((
        graphics_queue_family_index,
        transfer_queue_family_index,
    )))
}
