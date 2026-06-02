use crate::render::frame_graph::{FrameGraphResourceId, UnlitForwardPassNode};

impl UnlitForwardPassNode {
    /// Create a new [`UnlitForwardPassNode`]
    pub fn new(output: FrameGraphResourceId) -> UnlitForwardPassNode {
        UnlitForwardPassNode { output }
    }
}
