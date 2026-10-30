use crate::{
    Result,
    render::{GpuDataBuffers, HostDataBuffer, per_frame_data::data_buffers::DeviceDataBuffer},
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanBufferUsageFlags};
use colosseum_ecs::World;

impl GpuDataBuffers {
    /// Create a new uniform buffer
    pub fn create_uniform<T: 'static>(
        &mut self,
        name: &str,
        capacity: usize,
        world: &mut World,
    ) -> Result<()> {
        self.create::<T, _>(name, capacity, VulkanBufferUsageFlag::UniformBuffer, world)
    }

    /// Create a new uniform buffer with some initial data
    pub fn create_uniform_with_data<T: 'static + Clone>(
        &mut self,
        name: &str,
        initial_data: &[T],
        world: &mut World,
    ) -> Result<()> {
        self.create_with_data(
            name,
            VulkanBufferUsageFlag::UniformBuffer,
            initial_data,
            world,
        )
    }

    /// Create a new storage buffer
    pub fn create_storage<T: 'static>(
        &mut self,
        name: &str,
        capacity: usize,
        world: &mut World,
    ) -> Result<()> {
        self.create::<T, _>(name, capacity, VulkanBufferUsageFlag::StorageBuffer, world)
    }

    /// Create a new data buffer
    fn create_with_data<T: 'static + Clone, F: Into<VulkanBufferUsageFlags>>(
        &mut self,
        name: &str,
        usage: F,
        initial_data: &[T],
        world: &mut World,
    ) -> Result<()> {
        // Create the host data buffer
        let host_data_buffer = HostDataBuffer::<T>::with_data(name, initial_data, &self.device)?;

        // Insert the host data buffer into the world and get its index
        world.insert_permanent_resource(host_data_buffer);
        let host_index = world.get_or_reserve_permanent_resource_index::<HostDataBuffer<T>>();

        // Create the device data buffer
        let device_data_buffer = DeviceDataBuffer::<T>::new(
            name,
            initial_data.len(),
            usage.into(),
            host_index,
            &self.device,
        )?;
        self.buffers.push(Box::new(device_data_buffer));

        Ok(())
    }

    /// Create a new data buffer
    fn create<T: 'static, F: Into<VulkanBufferUsageFlags>>(
        &mut self,
        name: &str,
        capacity: usize,
        usage: F,
        world: &mut World,
    ) -> Result<()> {
        // Create and insert the host data buffer
        let host_data_buffer = HostDataBuffer::<T>::new(name, capacity, &self.device)?;
        world.insert_permanent_resource(host_data_buffer);
        let host_index = world.get_or_reserve_permanent_resource_index::<HostDataBuffer<T>>();

        // Create the device data buffer
        let device_data_buffer =
            DeviceDataBuffer::<T>::new(name, capacity, usage.into(), host_index, &self.device)?;
        self.buffers.push(Box::new(device_data_buffer));

        Ok(())
    }
}
