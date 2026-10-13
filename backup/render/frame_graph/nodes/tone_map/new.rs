use crate::render::frame_graph::{FrameGraphResourceId, ToneMapNode};

impl ToneMapNode {
    /// Create a new [`ToneMapNode`]
    pub(in crate::render::frame_graph) fn new(
        input: FrameGraphResourceId,
        output: FrameGraphResourceId,
    ) -> ToneMapNode {
        ToneMapNode { input, output }
    }
}
