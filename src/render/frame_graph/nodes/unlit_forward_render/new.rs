use crate::render::frame_graph::{FrameGraphResourceId, UnlitForwardRenderNode};

impl UnlitForwardRenderNode {
    /// Create a new [`UnlitForwardRenderNode`]
    pub fn new(output: FrameGraphResourceId) -> UnlitForwardRenderNode {
        UnlitForwardRenderNode { output }
    }
}
