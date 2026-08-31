use crate::render::data::LocalDataBuffer;

impl<T> LocalDataBuffer<T> {
    /// Push a new element to the data buffer
    pub fn push(&mut self, data: T) -> usize {
        debug_assert!(self.count < self.capacity);

        let index = self.count;
        self.memory[index] = data;
        self.count += 1;
        index
    }
}
