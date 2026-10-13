use crate::render::LocalDataBuffer;

impl<T> LocalDataBuffer<T> {
    /// Push a new element to the data buffer
    pub(in crate::render::data) fn push(&mut self, data: T) -> usize {
        debug_assert!(self.count < self.capacity);

        let index = self.count;
        self.memory[index] = data;
        self.count += 1;
        index
    }
}
