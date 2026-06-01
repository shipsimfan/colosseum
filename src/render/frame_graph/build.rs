use crate::render::{FrameContext, FrameGraph, RenderData, frame_graph::UnlitForwardPassNode};

impl FrameGraph {
    /// Build, compile, and execute the frame graph
    pub fn build(&mut self, data: &RenderData, frame: FrameContext) {
        // TODO: Add nodes to the frame graph here
        self.add_node(UnlitForwardPassNode::new(
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
        ));

        // Compile and execute the frame graph
        self.compile();
        self.execute(frame);
    }
}
