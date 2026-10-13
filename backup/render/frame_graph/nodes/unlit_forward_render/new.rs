use crate::render::frame_graph::{FrameGraphResourceId, UnlitForwardRenderNode};

impl UnlitForwardRenderNode {
    /// Create a new [`UnlitForwardRenderNode`]
    pub fn new(
        output: FrameGraphResourceId,
        depth_buffer: FrameGraphResourceId,
    ) -> UnlitForwardRenderNode {
        UnlitForwardRenderNode {
            output,
            depth_buffer,
        }
    }
}
