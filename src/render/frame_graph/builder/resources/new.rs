use crate::render::{
    DEPTH_FORMAT,
    frame_graph::{
        FrameGraphExternalResourceInfo, FrameGraphTransientResourceInfo, ShadowMapData,
        builder::FrameGraphBuilderResources,
    },
};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl FrameGraphBuilderResources {
    /// Create a new set of [`FrameGraphBuilderResources`]
    pub fn new() -> FrameGraphBuilderResources {
        FrameGraphBuilderResources {
            external_resources: vec![
                FrameGraphExternalResourceInfo::new(VulkanAttachmentLoadOp::DontCare), // Swapchain image
            ],

            native_scale_resources: Vec::new(),
            render_scale_resources: Vec::new(),
            shadow_map_resources: vec![
                FrameGraphTransientResourceInfo::new(
                    "Directional Light Shadow Maps",
                    DEPTH_FORMAT,
                    VulkanAttachmentLoadOp::Clear,
                    ShadowMapData::new(4, false),
                ),
                FrameGraphTransientResourceInfo::new(
                    "Point Light Shadow Maps",
                    DEPTH_FORMAT,
                    VulkanAttachmentLoadOp::Clear,
                    ShadowMapData::new(6, true),
                ),
                FrameGraphTransientResourceInfo::new(
                    "Spot Light Shadow Maps",
                    DEPTH_FORMAT,
                    VulkanAttachmentLoadOp::Clear,
                    ShadowMapData::new(1, false),
                ),
            ],
        }
    }
}
