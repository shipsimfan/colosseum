use crate::{
    Result,
    render::{DeviceDataBuffer, LocalDataBuffer},
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferCopy, VulkanCommandBuffer, VulkanDescriptorSet,
    VulkanDevice,
};

impl DeviceDataBuffer {
    /// Record the command to copy `local` to this buffer, binding a new buffer if needed
    pub fn copy<'a, T>(
        &'a mut self,
        local: &LocalDataBuffer<T>,
        created_descriptor_sets: &[VulkanDescriptorSet],
        cmd_buffer: &mut VulkanCommandBuffer,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        let local_capacity = local.capacity() * std::mem::size_of::<T>();
        if self.capacity < local_capacity {
            *self = DeviceDataBuffer::new_inner::<T>(
                self.buffer_name.clone(),
                self.memory_name.clone(),
                local.capacity(),
                self.usage,
                self.descriptor_type,
                self.descriptor_sets.clone(),
                created_descriptor_sets,
                device,
                memory_properties,
            )?;
        }

        let size = (local.count() * std::mem::size_of::<T>()) as _;
        if size > 0 {
            cmd_buffer.cmd_copy_buffer(
                local.buffer(),
                &self.buffer,
                &[VulkanBufferCopy::new(0, 0, size)],
            );
        }

        Ok(())
    }
}
