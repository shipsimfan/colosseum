use crate::render::frame_graph::FrameGraphTransientResourceInfo;
use alexandria::gpu::{
    VulkanImageAspectFlag, VulkanImageAspectFlags, VulkanImageUsageFlag, VulkanImageUsageFlags,
    VulkanSampler,
};
use std::sync::Arc;

impl FrameGraphTransientResourceInfo {
    /// Get the Vulkan image usage flags for this resource
    pub fn usage(&self) -> VulkanImageUsageFlags {
        let mut flags = VulkanImageUsageFlags::empty();

        if self.is_color {
            flags |= VulkanImageUsageFlag::ColorAttachment;
        }
        if self.is_depth {
            flags |= VulkanImageUsageFlag::DepthStencilAttachment;
        }
        if self.is_transfer_dst {
            flags |= VulkanImageUsageFlag::TransferDst;
        }
        if self.is_transfer_src {
            flags |= VulkanImageUsageFlag::TransferSrc;
        }
        if self.is_sampled_image {
            flags |= VulkanImageUsageFlag::Sampled;
        }

        flags
    }

    /// Get the Vulkan image aspect flags for this resource
    pub fn aspect_mask(&self) -> VulkanImageAspectFlags {
        let mut flags = VulkanImageAspectFlags::empty();

        if self.is_depth {
            flags |= VulkanImageAspectFlag::Depth;
        } else {
            flags |= VulkanImageAspectFlag::Color;
        }

        flags
    }

    /// Get the descriptor sets for this resource
    pub fn descriptor_sets(&self) -> &[(usize, u32, Arc<VulkanSampler>)] {
        &self.descriptor_sets
    }
}
