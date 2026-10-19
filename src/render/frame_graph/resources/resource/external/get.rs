use crate::render::frame_graph::FrameGraphExternalResource;
use alexandria::{
    gpu::{VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};

impl FrameGraphExternalResource {
    /// Get the image view of the external resource
    pub fn image_view(&self) -> &VulkanImageView {
        &self.image_view
    }

    /// Get the size of the external resource
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the number of layers of the external resource
    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }

    /// Get the aspect mask of the external resource
    pub fn aspect_mask(&self) -> VulkanImageAspectFlags {
        self.aspect_mask
    }
}
