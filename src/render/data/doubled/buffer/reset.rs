use crate::render::data::doubled::DataBuffer;

impl<T> DataBuffer<T> {
    /// Reset the buffer for a new frame
    pub fn reset(&mut self) {
        self.count = 0;
    }
}
