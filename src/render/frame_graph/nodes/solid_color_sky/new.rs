use crate::render::frame_graph::{FrameGraphResourceId, SolidColorSkyNode};

impl SolidColorSkyNode {
    /// Create a new [`SolidColorSkyNode`]
    pub(in crate::render::frame_graph) fn new(output: FrameGraphResourceId) -> SolidColorSkyNode {
        SolidColorSkyNode { output }
    }
}
