use crate::render::frame_graph::{FrameGraphResourceId, GammaCorrectionNode};

impl GammaCorrectionNode {
    /// Create a new [`GammaCorrectionNode`]
    pub(in crate::render::frame_graph) fn new(
        input: FrameGraphResourceId,
        output: FrameGraphResourceId,
    ) -> GammaCorrectionNode {
        GammaCorrectionNode { input, output }
    }
}
