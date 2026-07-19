use crate::{
    Result, ThreadManager,
    render::{CreatedRenderObject, GpuTransferQueue},
};
use alexandria::gpu::{VulkanDevice, VulkanQueue};
use std::sync::mpsc::{Sender, channel};

impl GpuTransferQueue {
    /// Create a new [`GpuTransferQueue`]
    pub(in crate::render) fn new(
        thread_manager: &ThreadManager,
        device: VulkanDevice,
        device_queue: VulkanQueue,
        staging_memory_type: usize,
        created_objects: Sender<CreatedRenderObject>,
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
                    created_objects,
                )
            },
            || {},
        )?;

        Ok(GpuTransferQueue { queue })
    }
}
