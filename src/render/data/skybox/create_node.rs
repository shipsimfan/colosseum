use crate::render::{FrameGraphNode, FrameGraphResourceId, Skybox, SolidColorSkyNode};

impl Skybox {
    /// Create a new [`FrameGraphNode`] for this skybox
    pub(in crate::render) fn create_node(&self, output: FrameGraphResourceId) -> FrameGraphNode {
        match self {
            Skybox::SolidColor(color) => {
                FrameGraphNode::SolidColorSky(SolidColorSkyNode::new(output, *color))
            }
        }
    }
}
