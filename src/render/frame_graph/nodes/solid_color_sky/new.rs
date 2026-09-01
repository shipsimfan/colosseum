use crate::render::frame_graph::{FrameGraphResourceId, SolidColorSkyNode};

impl SolidColorSkyNode {
    /// Create a new [`SolidColorSkyNode`]
    pub fn new(
        output: FrameGraphResourceId,
        depth_buffer: FrameGraphResourceId,
    ) -> SolidColorSkyNode {
        SolidColorSkyNode {
            output,
            depth_buffer,
        }
    }
}
