use crate::render::{MAX_FRAMES_IN_FLIGHT, frame_graph::resources::FrameGraphDataBinding};

impl FrameGraphDataBinding {
    /// Create a new instance of [`FrameGraphDataBinding`]
    pub fn new(index: usize, bindings: Vec<(usize, u32)>) -> FrameGraphDataBinding {
        FrameGraphDataBinding {
            epoch: [0; MAX_FRAMES_IN_FLIGHT],
            index,
            bindings,
        }
    }
}
