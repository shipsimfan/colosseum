use crate::render::{
    AntiAliasingMode, FrameGraph, HDR_FORMAT, SDR_FORMAT,
    frame_graph::{
        FrameGraphNode, FrameGraphResourceBuilder, FrameGraphResourceId, FrameGraphStructure,
        FxaaNode, QuantizationNode, RenderScaleNode, ToneMapNode, UnlitForwardRenderNode,
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
        let color_output = resources.create_render_scale_transient(HDR_FORMAT);

        // Perform the main render passes
        nodes.push(UnlitForwardRenderNode::new(color_output, depth_buffer).into());
        nodes.push(structure.skybox().create_node(color_output, depth_buffer));

        // Perform tone mapping
        let tone_map_output = resources.create_render_scale_transient(SDR_FORMAT);
        nodes.push(ToneMapNode::new(color_output, tone_map_output).into());

        // Perform render scaling, if needed
        let scaled_output = if structure.has_render_scale() {
            let scale_output = resources.create_native_scale_transient(SDR_FORMAT);
            nodes.push(RenderScaleNode::new(tone_map_output, scale_output).into());
            scale_output
        } else {
            tone_map_output
        };

        // Perform anti-aliasing
        let aa_output = match structure.anti_aliasing() {
            AntiAliasingMode::None => scaled_output,
            AntiAliasingMode::FXAA => {
                let aa_output = resources.create_native_scale_transient(SDR_FORMAT);
                nodes.push(FxaaNode::new(scaled_output, aa_output).into());
                aa_output
            }
        };

        // Quantize the output to the swapchain
        nodes.push(QuantizationNode::new(aa_output, FrameGraphResourceId::SWAPCHAIN_IMAGE).into());
    }
}
