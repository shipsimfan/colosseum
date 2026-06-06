use crate::render::frame_graph::resources::resource::FrameGraphResourceState;
use alexandria::gpu::VulkanImageLayout;

impl FrameGraphResourceState {
    /// Create a new [`FrameGraphResourceState`] with the default values
    pub fn new() -> FrameGraphResourceState {
        FrameGraphResourceState {
            stage_mask: 0.into(),
            access_mask: 0.into(),
            layout: VulkanImageLayout::Undefined,
        }
    }
}
