use crate::{
    Result,
    render::{
        SDR_FORMAT,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind,
            FrameGraphResourceUsage, RenderScaleNode,
        },
    },
};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl RenderScaleNode {
    /// Add a new [`RenderScaleNode`] to the frame graph
    pub fn add(
        input: FrameGraphResourceId,
        builder: &mut FrameGraphBuilder,
    ) -> Result<FrameGraphResourceId> {
        // Create output resource
        let output = builder.create_resource(
            "Render Scale Output",
            SDR_FORMAT,
            FrameGraphResourceKind::Native,
            VulkanAttachmentLoadOp::DontCare,
        );

        // Mark resource accesses
        builder.access(input, FrameGraphResourceUsage::TransferSrc);
        builder.access(output, FrameGraphResourceUsage::TransferDst);

        // Add node
        builder.add_node(RenderScaleNode { input, output }, true)?;

        Ok(output)
    }
}
