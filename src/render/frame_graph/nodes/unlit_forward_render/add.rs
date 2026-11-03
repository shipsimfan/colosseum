use crate::{
    Result,
    render::{
        CameraDsl, RenderableDsl,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage,
            UnlitForwardRenderNode,
        },
    },
};

impl UnlitForwardRenderNode {
    /// Add a new [`UnlitForwardRenderNode`] to the frame graph
    pub fn add(
        output: FrameGraphResourceId,
        depth_buffer: FrameGraphResourceId,
        builder: &mut FrameGraphBuilder,
    ) -> Result<()> {
        // Create descriptor set
        let camera_descriptor_set = builder.get_descriptor_set::<CameraDsl>();
        let renderable_descriptor_set = builder.get_descriptor_set::<RenderableDsl>();

        // Access the output as a color attachment and the depth buffer as a depth attachment
        builder.access(output, FrameGraphResourceUsage::ColorAttachment);
        builder.access(depth_buffer, FrameGraphResourceUsage::DepthAttachment);

        // Add the unlit forward render node to the frame graph
        builder.add_node(
            UnlitForwardRenderNode {
                camera_descriptor_set,
                renderable_descriptor_set,
            },
            true,
        )
    }
}
