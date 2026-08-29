use crate::render::data::doubled::DataBuffer;
use alexandria::gpu::GpuAddress;

impl<T> DataBuffer<T> {
    /// Push a new element to the data buffer
    pub fn push(&mut self, data: T) -> GpuAddress<T> {
        debug_assert!(self.count < self.capacity);

        let address = self.base_address.add(self.count);

        self.memory[self.count] = data;
        self.count += 1;

        address
    }
}
