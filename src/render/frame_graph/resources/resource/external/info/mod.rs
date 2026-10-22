use crate::render::frame_graph::FrameGraphResourceState;
use alexandria::gpu::VulkanAttachmentLoadOp;

mod new;
mod set;
mod transition;

/// Information about an external resource used within the frame graph
pub struct FrameGraphExternalResourceInfo {
    /// The load operation for the external resource
    load_op: VulkanAttachmentLoadOp,

    /// The current state of the resource
    state: FrameGraphResourceState,
}
