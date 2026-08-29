use crate::{
    Result,
    render::data::{DoubledRenderData, doubled::DataBuffer},
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl DoubledRenderData {
    /// Reserve enough space to store all renderables
    pub fn reserve_renderables(
        &mut self,
        num: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        if self.object_buffer.capacity() > num {
            return Ok(());
        }

        let mut new_capacity = self.object_buffer.capacity();
        while new_capacity < num {
            new_capacity *= 2;
        }

        self.object_buffer = DataBuffer::new(new_capacity, device, memory_properties)?;
        Ok(())
    }
}
