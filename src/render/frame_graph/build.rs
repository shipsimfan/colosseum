use crate::render::{
    FrameGraph,
    frame_graph::{
        FrameGraphNode, FrameGraphResourceBuilder, FrameGraphResourceId, FrameGraphStructure,
        UnlitForwardRenderNode,
    },
};
use alexandria::gpu::VulkanFormat;

impl FrameGraph {
    /// Build the frame graph for a single frame
    pub(in crate::render::frame_graph) fn build<'a>(
        structure: &FrameGraphStructure,

        resources: &mut FrameGraphResourceBuilder<'a>,
        nodes: &mut Vec<FrameGraphNode>,
    ) {
        // Reset the node list and transients
        nodes.clear();
        resources.clear_transient();

        // Create a common depth buffer
        let depth_buffer = resources.create_render_scale_transient(VulkanFormat::D32SFloat);

        // Add nodes to the frame graph
        nodes.push(
            UnlitForwardRenderNode::new(FrameGraphResourceId::SWAPCHAIN_IMAGE, depth_buffer).into(),
        );
        nodes.push(
            structure
                .skybox()
                .create_node(FrameGraphResourceId::SWAPCHAIN_IMAGE, depth_buffer),
        );
    }
}
