use crate::render::frame_graph::FrameGraphResourceId;
use alexandria::gpu::VulkanAttachmentLoadOp;

mod initialize;
mod new;
mod start;

/// The information needed to begin a render pass for a specific node
pub(in crate::render::frame_graph::nodes::system) struct FrameGraphRenderPass {
    /// The index of the frame graph resources
    resources: usize,

    /// The index of the frame index resource
    frame_index: usize,

    /// The ID of the color attachment for the render pass
    color_attachment: FrameGraphResourceId,

    /// The load operation for the color attachment
    color_attachment_load_op: VulkanAttachmentLoadOp,
}
