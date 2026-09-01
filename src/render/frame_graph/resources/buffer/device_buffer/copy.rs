use crate::{
    Result,
    render::{DeviceDataBuffer, LocalDataBuffer},
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferCopy, VulkanCommandBuffer, VulkanDevice,
};

impl DeviceDataBuffer {
    /// Record the command to copy `local` to this buffer, resizing if needed, returning if the buffer was resized
    pub fn reserve<T>(
        &mut self,
        local: &LocalDataBuffer<T>,
        cmd_buffer: &mut VulkanCommandBuffer,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<bool> {
        let local_capacity = local.capacity() * std::mem::size_of::<T>();
        let resize = self.capacity < local_capacity;
        if resize {
            *self = DeviceDataBuffer::new::<T, _>(
                local.capacity(),
                self.usage,
                device,
                memory_properties,
            )?;
        }

        let size = (local.count() * std::mem::size_of::<T>()) as _;
        cmd_buffer.cmd_copy_buffer(
            local.buffer(),
            &self.buffer,
            &[VulkanBufferCopy::new(0, 0, size)],
        );

        Ok(resize)
    }
}
