use crate::render::FrameHostDataBuffer;

impl<T> FrameHostDataBuffer<T> {
    /// Push a new element to the data buffer
    pub(in crate::render) fn push(&mut self, data: T) -> usize {
        debug_assert!(self.count < self.capacity);

        let index = self.count;
        self.memory[index] = data;
        self.count += 1;
        index
    }
}
