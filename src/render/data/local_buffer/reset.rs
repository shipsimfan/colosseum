use crate::render::data::LocalDataBuffer;

impl<T> LocalDataBuffer<T> {
    /// Reset the buffer for a new frame
    pub fn reset(&mut self) {
        self.count = 0;
    }
}
