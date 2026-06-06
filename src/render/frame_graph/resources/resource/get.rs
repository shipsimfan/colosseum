use crate::render::frame_graph::FrameGraphResource;
use alexandria::{
    gpu::{
        VulkanAccessFlags, VulkanFormat, VulkanImageLayout, VulkanImageView,
        VulkanPipelineStageFlags,
    },
    math::Vector2u,
};

impl<'a> FrameGraphResource<'a> {
    /// Get the image view of the resource
    pub const fn image_view(&self) -> &VulkanImageView {
        self.image_view.image_view()
    }

    /// Get the size of the resource, in pixels
    pub const fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the format of the resource
    pub const fn format(&self) -> VulkanFormat {
        self.format
    }

    pub fn state(
        &self,
    ) -> (
        VulkanPipelineStageFlags,
        VulkanAccessFlags,
        VulkanImageLayout,
    ) {
        self.state.borrow().get()
    }
}
