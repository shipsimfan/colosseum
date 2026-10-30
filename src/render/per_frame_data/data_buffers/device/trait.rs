use crate::{
    Result,
    render::{
        FrameIndex, GraphicsDevice, HostDataBuffer, per_frame_data::data_buffers::DeviceDataBuffer,
    },
};
use alexandria::gpu::{VulkanBufferCopy, VulkanCommandBuffer};
use colosseum_ecs::{World, WorldAccess};

/// A trait to erase the specific types of the device data buffers
pub trait TypelessDeviceDataBuffer {
    /// Set the access this device data buffer uses in the copy system
    fn access(&self, access: &mut WorldAccess);

    /// Copy the host data buffer to the device data buffer
    fn copy(
        &mut self,
        cmd_buffer: &mut VulkanCommandBuffer,
        frame_index: &FrameIndex,
        device: &GraphicsDevice,
        world: &mut World,
    ) -> Result<()>;
}

impl<T: 'static> TypelessDeviceDataBuffer for DeviceDataBuffer<T> {
    fn access(&self, access: &mut WorldAccess) {
        access.read_resource::<HostDataBuffer<T>>();
    }

    fn copy(
        &mut self,
        cmd_buffer: &mut VulkanCommandBuffer,
        frame_index: &FrameIndex,
        device: &GraphicsDevice,
        world: &mut World,
    ) -> Result<()> {
        let host_data_buffer =
            unsafe { world.get_resource_at_index::<HostDataBuffer<T>>(self.host_index) };
        let host_data_buffer = &host_data_buffer[frame_index];

        let device_data_buffer = &mut self.frame[**frame_index];

        // Check if we need to resize
        if host_data_buffer.count() > device_data_buffer.capacity() {
            device_data_buffer.resize(host_data_buffer.capacity(), device)?;
        }

        // Copy the host data buffer to the device data buffer
        if host_data_buffer.count() > 0 {
            cmd_buffer.cmd_copy_buffer(
                host_data_buffer.buffer(),
                device_data_buffer.buffer(),
                &[VulkanBufferCopy::new(0, 0, host_data_buffer.size() as u64)],
            );
        }

        Ok(())
    }
}
