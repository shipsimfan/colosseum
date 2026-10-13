use crate::render::ShadowMapBuffer;
use alexandria::{
    gpu::{VulkanImage, VulkanImageView},
    math::Vector2u,
};

impl ShadowMapBuffer {
    /// Get the handle to underlying image
    pub fn image(&self) -> &VulkanImage {
        &self.image
    }

    /// Get an image view for a specific light
    pub fn image_view(&self, index: usize) -> &VulkanImageView {
        &self.layer_image_views[index]
    }

    /// Get the size of the contained shadow maps
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the number of layers in the shadow map buffer
    pub fn layer_count(&self) -> u32 {
        (self.layer_image_views.len() * self.cascades) as u32
    }
}
