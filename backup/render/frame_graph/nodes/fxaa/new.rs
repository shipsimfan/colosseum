use crate::render::frame_graph::{FrameGraphResourceId, FxaaNode};

impl FxaaNode {
    /// Create a new [`FxaaNode`]
    pub fn new(input: FrameGraphResourceId, output: FrameGraphResourceId) -> FxaaNode {
        FxaaNode { input, output }
    }
}
