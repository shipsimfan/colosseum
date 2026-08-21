use crate::render::frame_graph::{
    FrameGraphNode, FrameGraphResourceId, FrameGraphSkybox, SolidColorSkyNode,
};

impl FrameGraphSkybox {
    /// Create a new [`FrameGraphNode`] for this skybox
    pub fn create_node(
        &self,
        output: FrameGraphResourceId,
        depth_buffer: FrameGraphResourceId,
    ) -> FrameGraphNode {
        match self {
            FrameGraphSkybox::SolidColor => SolidColorSkyNode::new(output, depth_buffer).into(),
        }
    }
}
