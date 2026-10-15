use crate::render::FrameIndex;

impl FrameIndex {
    /// Create a new [`FrameIndex`] starting at 0
    pub fn new() -> FrameIndex {
        FrameIndex { index: 0 }
    }
}
