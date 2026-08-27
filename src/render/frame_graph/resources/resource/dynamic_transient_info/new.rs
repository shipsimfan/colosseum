use crate::render::frame_graph::{FrameGraphDynamicTransientResourceInfo, FrameGraphResourceState};
use alexandria::gpu::VulkanFormat;

impl FrameGraphDynamicTransientResourceInfo {
    /// Create a new [`FrameGraphDynamicTransientResourceInfo`]
    pub(in crate::render::frame_graph::resources) fn new(
        format: VulkanFormat,
    ) -> FrameGraphDynamicTransientResourceInfo {
        FrameGraphDynamicTransientResourceInfo {
            format,
            is_color: false,
            is_depth: false,
            is_transfer_dst: false,
            is_transfer_src: false,
            is_sampled_image: false,
            state: FrameGraphResourceState::default(),
        }
    }
}
