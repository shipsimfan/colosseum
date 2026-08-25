use crate::{
    Error, Result, ThreadManager, info,
    logging::Logger,
    render::{
        FrameGraph, GpuTransferQueue, RenderData, RenderObjects,
        job::{GraphicsDevice, graphics_device::VulkanAdapterInfo},
    },
    warning,
};
use alexandria::gpu::{
    VulkanCommandPoolCreateFlag, VulkanDeviceBufferDeviceAddressFeatures,
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceExtension, VulkanDeviceVulkan11Features,
    VulkanDeviceVulkan13Features, VulkanInstance, VulkanQueueCreateInfo, VulkanSurface,
};

impl GraphicsDevice {
    /// Locate a new graphics device based on the name or UUID of the adapter, or use a default if none is provided
    pub fn new(
        adapter: Option<&str>,
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        logger: &Logger,
        thread_manager: &ThreadManager,
    ) -> Result<(GraphicsDevice, GpuTransferQueue)> {
        let logger = logger.logger("vulkan");

        // Select the adapter
        let adapter = select_adapter(adapter, instance, &surface, &logger)?;

        // Create the Vulkan device
        let mut vulkan_11_features =
            VulkanDeviceVulkan11Features::default().enable_shader_draw_parameters();
        let mut vulkan_13_features = VulkanDeviceVulkan13Features::default()
            .enable_synchronization2()
            .enable_dynamic_rendering();
        let mut buffer_device_address_features =
            VulkanDeviceBufferDeviceAddressFeatures::default().enable_buffer_device_address();
        let mut extended_dynamic_state_features =
            VulkanDeviceExtendedDynamicStateFeatures::default().enable_extended_dynamic_state();

        let mut device_builder = adapter.device_builder();
        device_builder
            .extension(VulkanDeviceExtension::Swapchain)
            .queue(VulkanQueueCreateInfo::new(
                adapter.graphics_queue_family_index(),
                &[1.0],
            ))
            .feature(&mut vulkan_11_features)
            .feature(&mut vulkan_13_features)
            .feature(&mut buffer_device_address_features)
            .feature(&mut extended_dynamic_state_features);
        if let Some(transfer_queue_family_index) = adapter.transfer_queue_family_index() {
            device_builder.queue(VulkanQueueCreateInfo::new(
                transfer_queue_family_index,
                &[1.0],
            ));
        }

        let (device, mut queues) = device_builder.create().map_err(Error::new_inner)?;

        let mut queue = queues.swap_remove(0);

        // Create the command pool
        let command_pool = device
            .create_command_pool(
                queue.queue_family(),
                VulkanCommandPoolCreateFlag::ResetCommandBuffer,
            )
            .map_err(Error::new_inner)?;

        // Create the transfer queue
        let (transfer_queue, gpu_transfer_queue) = match adapter.transfer_queue_family_index() {
            Some(_) => (
                GpuTransferQueue::new_dedicated(
                    thread_manager,
                    &device,
                    queues.swap_remove(0),
                    adapter.memory_properties(),
                    &logger,
                )?,
                None,
            ),
            None => {
                let (transfer_queue, gpu_transfer_queue) =
                    GpuTransferQueue::new(&device, &mut queue, adapter.memory_properties())?;
                (transfer_queue, Some(gpu_transfer_queue))
            }
        };

        // Create the render objects
        let render_objects = RenderObjects::new(adapter.swapchain_format(), &device)?;

        // Create the initial render data
        let render_data = vec![RenderData::new(
            &device,
            adapter.memory_properties(),
            &render_objects,
        )?];

        Ok((
            GraphicsDevice {
                logger: logger.clone(),
                device,
                queue,
                command_pool,
                command_buffers: Vec::new(),
                transient_buffers: Vec::new(),
                swapchain_format: adapter.swapchain_format(),
                frame_graph: FrameGraph::new(),
                render_objects,
                memory_properties: adapter.memory_properties().clone(),
                gpu_transfer_queue,
                render_data,
                current_render_data: 0,
            },
            transfer_queue,
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
        adapter.device_local_vram(),
        adapter.uuid()
    );
    Ok(adapter)
}
