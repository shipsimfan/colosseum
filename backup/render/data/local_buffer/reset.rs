use crate::render::LocalDataBuffer;

impl<T> LocalDataBuffer<T> {
    /// Reset the buffer for a new frame
    pub(in crate::render::data) fn reset(&mut self) {
        self.count = 0;
    }
}
