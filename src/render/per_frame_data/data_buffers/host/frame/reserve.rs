use crate::{
    Result,
    render::{FrameHostDataBuffer, GraphicsDevice},
};

impl<T> FrameHostDataBuffer<T> {
    /// Reserve enough capacity in the data buffer for `num` additional elements
    pub fn reserve(&mut self, num: usize, device: &GraphicsDevice) -> Result<()> {
        let desired_capacity = self.count + num;
        if self.capacity >= desired_capacity {
            return Ok(());
        }

        let mut new_capacity = self.capacity * 2;
        while new_capacity < desired_capacity {
            new_capacity *= 2;
        }

        let (buffer, memory, stride) = FrameHostDataBuffer::allocate_memory(
            &self.buffer_name,
            &self.memory_name,
            new_capacity,
            device,
        )?;

        self.buffer = buffer;
        self.memory = memory;
        self.stride = stride;

        Ok(())
    }
}
