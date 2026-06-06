use crate::render::{
    FrameGraph,
    frame_graph::{
        FrameGraphNode, FrameGraphResourceBuilder, FrameGraphResourceId, FrameGraphStructure,
        UnlitForwardRenderNode,
    },
};

impl FrameGraph {
    /// Build the frame graph for a single frame
    pub(in crate::render::frame_graph) fn build<'a>(
        structure: &FrameGraphStructure,

        resources: &mut FrameGraphResourceBuilder<'a>,
        nodes: &mut Vec<FrameGraphNode>,
    ) {
        // Reset the node list
        nodes.clear();

        // Add nodes to the frame graph
        nodes.push(
            structure
                .skybox()
                .create_node(FrameGraphResourceId::SWAPCHAIN_IMAGE),
        );
        nodes.push(UnlitForwardRenderNode::new(FrameGraphResourceId::SWAPCHAIN_IMAGE).into());
    }
}
