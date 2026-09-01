use crate::render::frame_graph::{FrameGraphResourceId, RenderScaleNode};

impl RenderScaleNode {
    /// Create a new [`RenderScaleNode`]
    pub fn new(input: FrameGraphResourceId, output: FrameGraphResourceId) -> RenderScaleNode {
        RenderScaleNode { input, output }
    }
}
