use crate::render::frame_graph::{FrameGraphResourceId, nodes::system::FrameGraphRenderPass};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl FrameGraphRenderPass {
    /// Create a new [`FrameGraphRenderPass`]
    pub fn new(
        color_attachment: Option<(FrameGraphResourceId, VulkanAttachmentLoadOp)>,
        depth_attachment: Option<(FrameGraphResourceId, VulkanAttachmentLoadOp)>,
    ) -> Option<FrameGraphRenderPass> {
        if color_attachment.is_none() && depth_attachment.is_none() {
            return None;
        }

        if depth_attachment
            .map(|(id, _)| id.is_shadow_map())
            .unwrap_or(false)
        {
            debug_assert!(color_attachment.is_none());
            return None;
        }

        Some(FrameGraphRenderPass {
            resources: 0,
            frame_index: 0,
            color_attachment,
            depth_attachment,
        })
    }
}
