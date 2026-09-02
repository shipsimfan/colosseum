use crate::{
    Error, Result,
    render::{DeviceDataBuffer, PerFrameObjectBuilder},
};
use alexandria::gpu::{VulkanBufferUsageFlags, VulkanDescriptorType};

impl<'a> PerFrameObjectBuilder<'a> {
    /// Add a new per-frame descriptor set
    pub fn add_descriptor_set(&mut self, descriptor_set_layout: usize, index: usize) -> Result<()> {
        assert_eq!(index, self.descriptor_sets.len());

        let descriptor_set_layout = self
            .fixed_render_objects
            .descriptor_set_layout(descriptor_set_layout);

        let descriptor_set = self
            .descriptor_pool
            .allocate_descriptor_set(descriptor_set_layout)
            .map_err(Error::new_inner)?;

        self.descriptor_sets.push(descriptor_set);
        Ok(())
    }

    /// Add a new per-frame device data buffer
    pub fn add_device_data_buffer<T, U: Into<VulkanBufferUsageFlags>>(
        &mut self,
        initial_capacity: usize,
        usage: U,

        descriptor_set: usize,
        descriptor_type: VulkanDescriptorType,
        binding: u32,

        index: usize,
    ) -> Result<()> {
        assert_eq!(index, self.device_buffers.len());

        let device_buffer = DeviceDataBuffer::new::<T>(
            initial_capacity,
            usage.into(),
            &self.descriptor_sets[descriptor_set],
            descriptor_type,
            binding,
            self.device,
            self.memory_properties,
        )?;
        self.device_buffers.push(device_buffer);

        Ok(())
    }
}
