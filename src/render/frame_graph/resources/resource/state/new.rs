use crate::render::frame_graph::resources::resource::FrameGraphResourceState;
use alexandria::gpu::{VulkanImageLayout, VulkanPipelineStageFlag};

impl FrameGraphResourceState {
    /// Create a new [`FrameGraphResourceState`] with the default values
    pub fn new() -> FrameGraphResourceState {
        FrameGraphResourceState {
            stage_mask: VulkanPipelineStageFlag::ColorAttachmentOutput.into(),
            access_mask: 0.into(),
            layout: VulkanImageLayout::Undefined,
        }
    }
}
