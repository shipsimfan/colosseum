use crate::{
    Error, Result, info,
    logging::Logger,
    render::{
        FrameGraph, RenderObjects,
        job::{GraphicsDevice, graphics_device::VulkanAdapterInfo},
    },
    warning,
};
use alexandria::gpu::{
    VulkanCommandPoolCreateFlag, VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceExtension,
    VulkanDeviceVulkan11Features, VulkanDeviceVulkan13Features, VulkanInstance,
    VulkanQueueCreateInfo, VulkanSurface,
};

impl GraphicsDevice {
    /// Locate a new graphics device based on the name or UUID of the adapter, or use a default if none is provided
    pub fn new(
        adapter: Option<&str>,
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        logger: &Logger,
    ) -> Result<GraphicsDevice> {
        let logger = logger.logger("vulkan");

        // Select the adapter
        let adapter = select_adapter(adapter, instance, &surface, &logger)?;

        // Create the Vulkan device
        let (device, mut queues) = adapter
            .device_builder()
            .extension(VulkanDeviceExtension::Swapchain)
            .queue(VulkanQueueCreateInfo::new(
                adapter.graphics_queue_family_index(),
                &[1.0],
            ))
            .feature(&mut VulkanDeviceVulkan11Features::default().enable_shader_draw_parameters())
            .feature(
                &mut VulkanDeviceVulkan13Features::default()
                    .enable_synchronization2()
                    .enable_dynamic_rendering(),
            )
            .feature(
                &mut VulkanDeviceExtendedDynamicStateFeatures::default()
                    .enable_extended_dynamic_state(),
            )
            .create()
            .map_err(Error::new_inner)?;

        let queue = queues.swap_remove(0);

        // Create the command pool
        let command_pool = device
            .create_command_pool(
                queue.queue_family(),
                VulkanCommandPoolCreateFlag::ResetCommandBuffer,
            )
            .map_err(Error::new_inner)?;

        Ok(GraphicsDevice {
            logger: logger.clone(),
            device,
            queue,
            command_pool,
            command_buffers: Vec::new(),
            swapchain_format: adapter.swapchain_format(),
            frame_graph: FrameGraph::new(),
            render_objects: RenderObjects::new(),
        })
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
    let mut adapters = GraphicsDevice::get_adapters(instance, surface, Some(&logger))?;
    if adapters.len() == 0 {
        return Err(Error::new("no compatible graphics adapters found"));
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
        adapter.vram(),
        adapter.uuid()
    );
    Ok(adapter)
}
