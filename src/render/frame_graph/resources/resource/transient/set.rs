use crate::render::frame_graph::FrameGraphTransientResourceInfo;
use alexandria::gpu::VulkanAttachmentLoadOp;

impl FrameGraphTransientResourceInfo {
    /// Set this resource to be a color resource
    pub fn set_color(&mut self) -> VulkanAttachmentLoadOp {
        assert!(!self.is_depth);
        self.is_color = true;
        self.load_op()
    }

    /// Set this resource to be a depth resource
    pub fn set_depth(&mut self) -> VulkanAttachmentLoadOp {
        assert!(!self.is_color);
        self.is_depth = true;
        self.load_op()
    }

    /// Set this resource to be a transfer destination resource
    pub fn set_transfer_dst(&mut self) {
        self.is_transfer_dst = true;
    }

    /// Set this resource to be a transfer source resource
    pub fn set_transfer_src(&mut self) {
        self.is_transfer_src = true;
    }

    /// Set this resource to be a sampled image resource
    pub fn set_sampled_image(&mut self) {
        self.is_sampled_image = true;
    }

    fn load_op(&mut self) -> VulkanAttachmentLoadOp {
        let load_op = self.load_op;
        self.load_op = VulkanAttachmentLoadOp::Load;
        load_op
    }
}
