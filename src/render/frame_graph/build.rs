use crate::render::{
    FrameGraph, SDR_FORMAT,
    frame_graph::{
        FrameGraphNode, FrameGraphResourceBuilder, FrameGraphResourceId, FrameGraphStructure,
        GammaCorrectionNode, RenderScaleNode, UnlitForwardRenderNode,
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

        // Create the 3d color output
        let color_output = resources.create_render_scale_transient(SDR_FORMAT);

        // Add nodes to the frame graph
        nodes.push(UnlitForwardRenderNode::new(color_output, depth_buffer).into());
        nodes.push(structure.skybox().create_node(color_output, depth_buffer));

        let scaled_output = if structure.has_render_scale() {
            let scale_output = resources.create_native_scale_transient(SDR_FORMAT);
            nodes.push(RenderScaleNode::new(color_output, scale_output).into());
            scale_output
        } else {
            color_output
        };

        nodes.push(
            GammaCorrectionNode::new(scaled_output, FrameGraphResourceId::SWAPCHAIN_IMAGE).into(),
        );
    }
}
