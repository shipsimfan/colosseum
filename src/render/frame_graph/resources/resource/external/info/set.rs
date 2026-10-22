use crate::render::frame_graph::FrameGraphExternalResourceInfo;
use alexandria::gpu::VulkanAttachmentLoadOp;

impl FrameGraphExternalResourceInfo {
    /// Set that the external resource is being used as a color attachment
    pub fn set_color(&mut self) -> VulkanAttachmentLoadOp {
        let load_op = self.load_op;
        self.load_op = VulkanAttachmentLoadOp::Load;
        load_op
    }
}
