use crate::render::frame_graph::{FrameGraphResourceId, ProceduralSkyNode};

impl ProceduralSkyNode {
    /// Create a new [`ProceduralSkyNode`]
    pub fn new(output: FrameGraphResourceId) -> ProceduralSkyNode {
        ProceduralSkyNode { output }
    }
}
