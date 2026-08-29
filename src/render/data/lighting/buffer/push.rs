use crate::render::data::lighting::LightingDataBuffer;

impl<T> LightingDataBuffer<T> {
    /// Push a new element to the data buffer
    pub fn push(&mut self, data: T) {
        debug_assert!(self.count < self.capacity);

        self.memory[self.count] = data;
        self.count += 1;
    }
}
