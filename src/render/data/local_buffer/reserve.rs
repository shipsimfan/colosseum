use crate::{Result, render::LocalDataBuffer};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl<T> LocalDataBuffer<T> {
    /// Reserve enough capacity in the data buffer for `num` elements, returning if the buffer was
    /// reallocated
    pub(in crate::render::data) fn reserve(
        &mut self,
        num: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        if self.capacity >= num {
            return Ok(());
        }

        let mut new_capacity = self.capacity * 2;
        while new_capacity < num {
            new_capacity *= 2;
        }

        #[allow(invalid_value)]
        let mut name = unsafe { std::mem::zeroed() };
        std::mem::swap(&mut name, &mut self.name);

        *self = LocalDataBuffer::new_inner(name, new_capacity, device, memory_properties)?;
        Ok(())
    }
}
