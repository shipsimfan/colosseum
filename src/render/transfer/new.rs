use crate::{Result, ThreadManager, logging::Logger, render::GpuTransferQueue};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanQueue};
use std::sync::{Arc, mpsc::channel};

impl GpuTransferQueue {
    /// Create a new [`GpuTransferQueue`]
    pub(in crate::render) fn new(
        thread_manager: &ThreadManager,
        device: VulkanDevice,
        device_queue: VulkanQueue,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
        logger: &Logger,
    ) -> Result<GpuTransferQueue> {
        let (queue, receiver) = channel();

        let logger = logger.logger("gpu-transfer");
        thread_manager.spawn(
            format!("GPU Transfer"),
            move |shared_state| {
                GpuTransferQueue::thread(
                    shared_state,
                    receiver,
                    device,
                    device_queue,
                    memory_properties,
                    logger,
                )
            },
            || {},
        )?;

        Ok(GpuTransferQueue { queue })
    }
}
