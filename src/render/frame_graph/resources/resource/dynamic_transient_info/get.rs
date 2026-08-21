use crate::render::frame_graph::FrameGraphDynamicTransientResourceInfo;
use alexandria::gpu::{
    VulkanFormat, VulkanImageAspectFlag, VulkanImageAspectFlags, VulkanImageUsageFlag,
    VulkanImageUsageFlags,
};

impl FrameGraphDynamicTransientResourceInfo {
    /// Get the format of the resource
    pub fn format(&self) -> VulkanFormat {
        self.format
    }

    /// Get the Vulkan image usage flags for this resource
    pub fn usage(&self) -> VulkanImageUsageFlags {
        let mut flags = VulkanImageUsageFlags::empty();

        if self.is_color {
            flags |= VulkanImageUsageFlag::ColorAttachment;
        }
        if self.is_depth {
            flags |= VulkanImageUsageFlag::DepthStencilAttachment;
        }

        flags
    }

    /// Get the Vulkan image aspect flags for this resource
    pub fn aspect_mask(&self) -> VulkanImageAspectFlags {
        let mut flags = VulkanImageAspectFlags::empty();

        if self.is_color {
            flags |= VulkanImageAspectFlag::Color;
        }
        if self.is_depth {
            flags |= VulkanImageAspectFlag::Depth;
        }

        flags
    }
}
