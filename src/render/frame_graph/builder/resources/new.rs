use crate::render::frame_graph::{
    FrameGraphExternalResourceInfo, builder::FrameGraphBuilderResources,
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
        }
    }
}
