use crate::render::frame_graph::{FrameGraphResourceId, RenderScaleNode};

impl RenderScaleNode {
    /// Create a new [`RenderScaleNode`]
    pub(in crate::render::frame_graph) fn new(
        input: FrameGraphResourceId,
        output: FrameGraphResourceId,
    ) -> RenderScaleNode {
        RenderScaleNode { input, output }
    }
}
