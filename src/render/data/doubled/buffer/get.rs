use crate::render::data::doubled::DataBuffer;

impl<T> DataBuffer<T> {
    /// Get the capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
