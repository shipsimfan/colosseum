use crate::render::frame_graph::{FrameGraphResourceId, FxaaNode};

impl FxaaNode {
    /// Create a new [`FxaaNode`]
    pub(in crate::render::frame_graph) fn new(
        input: FrameGraphResourceId,
        output: FrameGraphResourceId,
    ) -> FxaaNode {
        FxaaNode { input, output }
    }
}
