use crate::render::frame_graph::{FrameGraphResourceState, FrameGraphResourceUsage};
use alexandria::gpu::{VulkanAccessFlag, VulkanImageLayout, VulkanPipelineStageFlag};

impl<'a> FrameGraphResourceUsage<'a> {
    /// Get the state the resource should be in for this usage and set the usage flags for a resource
    pub fn state(&self) -> FrameGraphResourceState {
        match self {
            FrameGraphResourceUsage::ColorAttachment => FrameGraphResourceState::new(
                VulkanPipelineStageFlag::ColorAttachmentOutput,
                VulkanAccessFlag::ColorAttachmentWrite,
                VulkanImageLayout::ColorAttachmentOptimal,
            ),
            FrameGraphResourceUsage::DepthAttachment
            | FrameGraphResourceUsage::ShadowMapDepthAttachment => FrameGraphResourceState::new(
                VulkanPipelineStageFlag::EarlyFragmentTests,
                VulkanAccessFlag::DepthStencilAttachmentWrite,
                VulkanImageLayout::DepthStencilAttachmentOptimal,
            ),
            FrameGraphResourceUsage::TransferDst => FrameGraphResourceState::new(
                VulkanPipelineStageFlag::Blit,
                VulkanAccessFlag::TransferWrite,
                VulkanImageLayout::TransferDstOptimal,
            ),
            FrameGraphResourceUsage::TransferSrc => FrameGraphResourceState::new(
                VulkanPipelineStageFlag::Blit,
                VulkanAccessFlag::TransferRead,
                VulkanImageLayout::TransferSrcOptimal,
            ),
            FrameGraphResourceUsage::SampledImage { .. } => FrameGraphResourceState::new(
                VulkanPipelineStageFlag::FragmentShader,
                VulkanAccessFlag::ShaderRead,
                VulkanImageLayout::ShaderReadOnlyOptimal,
            ),
        }
    }
}
