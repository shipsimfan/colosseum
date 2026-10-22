use crate::render::frame_graph::{FrameGraphExternalResourceInfo, FrameGraphResourceState};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl FrameGraphExternalResourceInfo {
    /// Creates a new [`FrameGraphExternalResourceInfo`]
    pub fn new(initial_load_lop: VulkanAttachmentLoadOp) -> FrameGraphExternalResourceInfo {
        FrameGraphExternalResourceInfo {
            load_op: initial_load_lop,
            state: FrameGraphResourceState::default(),
        }
    }
}
