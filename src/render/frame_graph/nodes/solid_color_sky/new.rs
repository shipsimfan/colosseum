use crate::render::frame_graph::{FrameGraphResourceId, SolidColorSkyNode};
use alexandria::math::{Color3f, Linear};

impl SolidColorSkyNode {
    /// Create a new [`SolidColorSkyNode`]
    pub fn new(output: FrameGraphResourceId, color: Color3f<Linear>) -> SolidColorSkyNode {
        SolidColorSkyNode { output, color }
    }
}
