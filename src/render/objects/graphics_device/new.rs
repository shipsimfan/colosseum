use crate::{
    Result, UserEvent,
    render::{
        GpuTransferQueue, GraphicsDevice, RenderGpuTransferQueue, RenderQueue,
        objects::graphics_device::VulkanAdapterInfo,
    },
};
use alexandria::gpu::{
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceExtension, VulkanDeviceFeatures,
    VulkanDeviceVulkan11Features, VulkanDeviceVulkan12Features, VulkanDeviceVulkan13Features,
    VulkanInstance, VulkanQueueCreateInfo, VulkanSurface,
};
use colosseum_core::{Logger, ThreadManager, info, logger, new_error, warning};
use std::sync::Arc;

impl GraphicsDevice {
    /// Locate a new graphics device based on the name or UUID of the adapter, or use a default if none is provided
    pub(in crate::render) fn new(
        adapter: Option<&str>,
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        thread_manager: &ThreadManager<UserEvent>,
    ) -> Result<(
        Arc<GraphicsDevice>,
        RenderQueue,
        GpuTransferQueue,
        Option<RenderGpuTransferQueue>,
    )> {
        let logger = logger!("vulkan");

        // Select the adapter
        let adapter = select_adapter(adapter, instance, &surface, &logger)?;

        // Create the Vulkan device
        let mut vulkan_10_features = VulkanDeviceFeatures::default().enable_image_cube_array();
        let mut vulkan_11_features =
            VulkanDeviceVulkan11Features::default().enable_shader_draw_parameters();
        let mut vulkan_12_features = VulkanDeviceVulkan12Features::default()
            .enable_shader_output_layer()
            .enable_timeline_semaphore();
        let mut vulkan_13_features = VulkanDeviceVulkan13Features::default()
            .enable_synchronization2()
            .enable_dynamic_rendering();
        let mut extended_dynamic_state_features =
            VulkanDeviceExtendedDynamicStateFeatures::default().enable_extended_dynamic_state();

        let mut device_builder = adapter.device_builder();
        device_builder
            .extension(VulkanDeviceExtension::Swapchain)
            .queue(VulkanQueueCreateInfo::new(
                adapter.graphics_queue_family_index(),
                &[1.0],
            ))
            .feature(&mut vulkan_10_features)
            .feature(&mut vulkan_11_features)
            .feature(&mut vulkan_12_features)
            .feature(&mut vulkan_13_features)
            .feature(&mut extended_dynamic_state_features);
        if let Some(transfer_queue_family_index) = adapter.transfer_queue_family_index() {
            device_builder.queue(VulkanQueueCreateInfo::new(
                transfer_queue_family_index,
                &[1.0],
            ));
        }

        let (device, mut queues) = device_builder
            .create()
            .map_err(|error| new_error!("unable to create Vulkan device - {}", error))?;

        let mut render_queue = queues.swap_remove(0);
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut render_queue, c"Render Queue")
            .map_err(|error| {
                new_error!("unable to set object name for render queue - {}", error)
            })?;

        // Create the transfer queue
        let (transfer_queue, gpu_transfer_queue) = match adapter.transfer_queue_family_index() {
            Some(_) => {
                let mut queue = queues.swap_remove(0);
                #[cfg(debug_assertions)]
                device
                    .set_object_name(&mut queue, c"Transfer Queue")
                    .map_err(|error| {
                        new_error!("unable to set object name for transfer queue - {}", error)
                    })?;

                (
                    GpuTransferQueue::new_dedicated(
                        thread_manager,
                        &device,
                        queue,
                        adapter.memory_properties(),
                    )?,
                    None,
                )
            }
            None => {
                let (transfer_queue, gpu_transfer_queue) =
                    GpuTransferQueue::new(&device, &mut render_queue, adapter.memory_properties())?;
                (transfer_queue, Some(gpu_transfer_queue))
            }
        };

        Ok((
            Arc::new(GraphicsDevice {
                logger: logger.clone(),
                device,
                render_queue_family: adapter.graphics_queue_family_index(),
                swapchain_format: adapter.swapchain_format(),
                memory_properties: adapter.memory_properties().clone(),
            }),
            RenderQueue::new(render_queue),
            transfer_queue,
            gpu_transfer_queue,
        ))
    }
}

/// Select a graphics adapter based on the provided name or UUID, or use a default if none is provided
fn select_adapter<'instance>(
    adapter: Option<&str>,
    instance: &'instance VulkanInstance,
    surface: &VulkanSurface,
    logger: &Logger,
) -> Result<VulkanAdapterInfo<'instance>> {
    // Get compatible adapters
    let mut adapters = GraphicsDevice::get_adapters(instance, surface, &logger)?;
    if adapters.len() == 0 {
        return Err(new_error!("no compatible graphics adapters found"));
    }

    // Check for an adapter matching the provided name or UUID
    let mut found_adapter = None;
    if let Some(adapter) = adapter {
        for (i, adapter_info) in adapters.iter().enumerate() {
            if adapter_info.name() == adapter || adapter_info.uuid().to_string() == adapter {
                info!(
                    logger,
                    "Found graphics adapter with UUID or name of \"{}\"", adapter
                );

                found_adapter = Some(i);
                break;
            }
        }
    }

    let adapter_index = match found_adapter {
        Some(adapter_index) => adapter_index,
        None => {
            match adapter {
                Some(adapter) => {
                    warning!(
                        logger,
                        "No compatible graphics adapter found with UUID or name of \"{}\", using default adapter",
                        adapter
                    );
                }
                None => {
                    info!(
                        logger,
                        "No graphics adapter specified, using default adapter"
                    );
                }
            }
            // If none found, use the first compatible adapter
            0
        }
    };

    // Remove the selected adapter from the list and return it
    let adapter = adapters.swap_remove(adapter_index);
    info!(
        logger,
        "Selected graphics adapter \"{}\" ({}, {})",
        adapter.name(),
        adapter.device_local_vram(),
        adapter.uuid()
    );
    Ok(adapter)
}
