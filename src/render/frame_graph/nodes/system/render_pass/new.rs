use crate::render::frame_graph::{FrameGraphResourceId, nodes::system::FrameGraphRenderPass};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl FrameGraphRenderPass {
    /// Create a new [`FrameGraphRenderPass`]
    pub fn new(
        color_attachment: Option<(FrameGraphResourceId, VulkanAttachmentLoadOp)>,
    ) -> Option<FrameGraphRenderPass> {
        let (color_attachment, color_attachment_load_op) = color_attachment?;

        Some(FrameGraphRenderPass {
            resources: 0,
            frame_index: 0,
            color_attachment,
            color_attachment_load_op,
        })
    }
}
