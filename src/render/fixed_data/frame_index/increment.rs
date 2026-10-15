use crate::render::{FrameIndex, MAX_FRAMES_IN_FLIGHT};

impl FrameIndex {
    /// Increment the frame index by 1
    pub fn increment(&mut self) {
        self.index = (self.index + 1) % MAX_FRAMES_IN_FLIGHT;
    }
}
