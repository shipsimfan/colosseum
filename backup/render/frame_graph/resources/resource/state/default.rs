use crate::render::frame_graph::FrameGraphResourceState;
use alexandria::gpu::VulkanImageLayout;

impl Default for FrameGraphResourceState {
    fn default() -> Self {
        FrameGraphResourceState::new(0, 0, VulkanImageLayout::Undefined)
    }
}
