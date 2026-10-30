use crate::{
    Result,
    render::{GraphicsDevice, per_frame_data::data_buffers::FrameDeviceDataBuffer},
};

impl<T> FrameDeviceDataBuffer<T> {
    /// Resize the buffer to the given capacity
    pub fn resize(&mut self, new_capacity: usize, device: &GraphicsDevice) -> Result<()> {
        if self.capacity == new_capacity {
            return Ok(());
        }

        let (buffer, memory) = FrameDeviceDataBuffer::<T>::allocate_memory(
            &self.buffer_name,
            &self.memory_name,
            new_capacity,
            self.usage,
            device,
        )?;

        self.buffer = buffer;
        self.memory = memory;
        self.capacity = new_capacity;

        Ok(())
    }
}
