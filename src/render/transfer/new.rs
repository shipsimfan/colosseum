use crate::{
    Result, ThreadManager,
    render::{CreatedRenderObject, GpuTransferQueue},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanQueue};
use std::sync::{
    Arc,
    mpsc::{Sender, channel},
};

impl GpuTransferQueue {
    /// Create a new [`GpuTransferQueue`]
    pub(in crate::render) fn new(
        thread_manager: &ThreadManager,
        device: VulkanDevice,
        device_queue: VulkanQueue,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
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
                    memory_properties,
                    created_objects,
                )
            },
            || {},
        )?;

        Ok(GpuTransferQueue { queue })
    }
}
