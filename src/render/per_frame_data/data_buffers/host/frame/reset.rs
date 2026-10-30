use crate::render::FrameHostDataBuffer;

impl<T> FrameHostDataBuffer<T> {
    /// Reset the buffer for a new frame
    pub fn reset(&mut self) {
        self.count = 0;
    }
}
