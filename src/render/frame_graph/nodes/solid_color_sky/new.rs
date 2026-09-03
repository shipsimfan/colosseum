use crate::render::frame_graph::{FrameGraphResourceId, SolidColorSkyNode};

impl SolidColorSkyNode {
    /// Create a new [`SolidColorSkyNode`]
    pub fn new(output: FrameGraphResourceId) -> SolidColorSkyNode {
        SolidColorSkyNode { output }
    }
}
