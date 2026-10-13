use crate::render::frame_graph::{FrameGraphResourceId, LitForwardRenderNode};

impl LitForwardRenderNode {
    /// Create a new [`LitForwardRenderNode`]
    pub fn new(
        output: FrameGraphResourceId,
        depth_buffer: FrameGraphResourceId,
    ) -> LitForwardRenderNode {
        LitForwardRenderNode {
            output,
            depth_buffer,
        }
    }
}
