use crate::render::{
    FrameGraph,
    frame_graph::{
        FrameGraphNode, FrameGraphResourceBuilder, FrameGraphResourceId, FrameGraphStructure,
        RenderScaleNode, UnlitForwardRenderNode,
    },
};
use alexandria::gpu::VulkanFormat;

impl FrameGraph {
    /// Build the frame graph for a single frame
    pub(in crate::render::frame_graph) fn build<'a>(
        structure: &FrameGraphStructure,
        swapchain_format: VulkanFormat,

        resources: &mut FrameGraphResourceBuilder<'a>,
        nodes: &mut Vec<FrameGraphNode>,
    ) {
        // Reset the node list and transients
        nodes.clear();
        resources.clear_transient();

        // Create a common depth buffer
        let depth_buffer = resources.create_render_scale_transient(VulkanFormat::D32SFloat);

        // Select the 3d color output
        let color_output = if structure.has_render_scale() {
            resources.create_render_scale_transient(swapchain_format)
        } else {
            FrameGraphResourceId::SWAPCHAIN_IMAGE
        };

        // Add nodes to the frame graph
        nodes.push(UnlitForwardRenderNode::new(color_output, depth_buffer).into());
        nodes.push(structure.skybox().create_node(color_output, depth_buffer));

        if structure.has_render_scale() {
            nodes.push(
                RenderScaleNode::new(color_output, FrameGraphResourceId::SWAPCHAIN_IMAGE).into(),
            );
        }
    }
}
