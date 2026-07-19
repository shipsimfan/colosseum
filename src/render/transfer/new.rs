use alexandria::gpu::{VulkanDevice, VulkanQueue};

use crate::{Result, ThreadManager, render::GpuTransferQueue};
use std::sync::mpsc::channel;

impl GpuTransferQueue {
    /// Create a new [`GpuTransferQueue`]
    pub(in crate::render) fn new(
        thread_manager: &ThreadManager,
        device: VulkanDevice,
        device_queue: VulkanQueue,
        staging_memory_type: usize,
    ) -> Result<GpuTransferQueue> {
        let (queue, receiver) = channel();

        thread_manager.spawn(
            format!("GPU Transfer"),
            move |shared_state| {
                GpuTransferQueue::thread(
                    shared_state,
                    receiver,
                    device,
                    device_queue,
                    staging_memory_type,
                )
            },
            || {},
        )?;

        Ok(GpuTransferQueue { queue })
    }
}
