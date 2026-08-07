use crate::{
    Result, ThreadManager,
    logging::Logger,
    render::{GpuTransferQueue, RenderGpuTransferQueue},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanQueue};
use std::sync::{Arc, mpsc::channel};

impl GpuTransferQueue {
    /// Create a new [`GpuTransferQueue`] with a dedicated transfer queue
    pub(in crate::render) fn new_dedicated(
        thread_manager: &ThreadManager,
        device: &VulkanDevice,
        mut device_queue: VulkanQueue,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
        logger: &Logger,
    ) -> Result<GpuTransferQueue> {
        let (transfer_queue, render_transfer_queue) =
            GpuTransferQueue::new(device, &mut device_queue, memory_properties)?;

        let logger = logger.logger("gpu-transfer");
        thread_manager.spawn(
            format!("GPU Transfer"),
            move |shared_state| {
                GpuTransferQueue::thread(shared_state, device_queue, render_transfer_queue, logger)
            },
            || {},
        )?;

        Ok(transfer_queue)
    }

    /// Create a new [`GpuTransferQueue`] without a dedicated transfer queue
    pub(in crate::render) fn new(
        device: &VulkanDevice,
        device_queue: &mut VulkanQueue,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<(GpuTransferQueue, RenderGpuTransferQueue)> {
        let (queue, receiver) = channel();

        let transfer_queue = GpuTransferQueue { queue };
        let render_transfer_queue =
            RenderGpuTransferQueue::new(device_queue, &memory_properties, device, receiver)?;

        Ok((transfer_queue, render_transfer_queue))
    }
}
