use crate::render::data::lighting::LightingDataBuffer;

impl<T> LightingDataBuffer<T> {
    /// Reset the buffer for a new frame
    pub fn reset(&mut self) {
        self.count = 0;
    }
}
