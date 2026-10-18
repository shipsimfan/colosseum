use std::sync::Arc;

use crate::render::Swapchain;
use alexandria::{
    gpu::{VulkanImage, VulkanImageView},
    math::Vector2u,
};

impl Swapchain {
    /// Is the swapchain currently valid?
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Is the swapchain currently suboptimal?
    pub fn is_suboptimal(&self) -> bool {
        self.suboptimal
    }

    /// Get a reference to the current swapchain image view
    pub fn image_view(&self) -> &Arc<VulkanImageView> {
        &self.image_views[self.image_index]
    }

    /// Get a reference to the current swapchain image
    pub fn image(&self) -> &VulkanImage {
        self.image_view().image()
    }

    /// Get the size of the swapchain images
    pub fn size(&self) -> Vector2u {
        self.size
    }
}
