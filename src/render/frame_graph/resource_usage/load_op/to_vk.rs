use crate::render::frame_graph::FrameGraphResourceLoadOp;
use alexandria::{
    gpu::VulkanAttachmentLoadOp,
    math::{Color4f, Linear},
};

impl FrameGraphResourceLoadOp {
    /// Convert this load operation to a Vulkan load operation and clear value
    pub fn to_vk(&self) -> (VulkanAttachmentLoadOp, Color4f<Linear>) {
        match self {
            FrameGraphResourceLoadOp::Clear { color } => (VulkanAttachmentLoadOp::Clear, *color),
            FrameGraphResourceLoadOp::Load => (VulkanAttachmentLoadOp::Load, Color4f::BLACK),
            FrameGraphResourceLoadOp::DontCare => {
                (VulkanAttachmentLoadOp::DontCare, Color4f::BLACK)
            }
        }
    }
}
