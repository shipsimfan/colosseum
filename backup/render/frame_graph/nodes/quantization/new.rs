use crate::render::frame_graph::{FrameGraphResourceId, QuantizationNode};

impl QuantizationNode {
    /// Create a new [`QuantizationNode`]
    pub fn new(input: FrameGraphResourceId, output: FrameGraphResourceId) -> QuantizationNode {
        QuantizationNode { input, output }
    }
}
