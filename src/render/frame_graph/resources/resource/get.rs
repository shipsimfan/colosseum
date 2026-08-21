use crate::render::frame_graph::FrameGraphResource;
use alexandria::{
    gpu::{VulkanImage, VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};

impl<'a, 'b> FrameGraphResource<'a, 'b> {
    /// Get the size of the resource
    pub fn size(&self) -> Vector2u {
        match self {
            FrameGraphResource::External(external) => external.size(),
            FrameGraphResource::Transient(transient) => transient.size(),
        }
    }

    /// Get the image associated with this resource
    pub fn image(&self) -> &'b VulkanImage {
        match self {
            FrameGraphResource::External(external) => external.image(),
            FrameGraphResource::Transient(transient) => transient.image(),
        }
    }

    /// Get the image view associated with this resource
    pub fn image_view(&self) -> &'b VulkanImageView {
        match self {
            FrameGraphResource::External(external) => external.image_view(),
            FrameGraphResource::Transient(transient) => transient.image_view(),
        }
    }

    /// Get the aspect mask associated with this resource
    pub fn aspect_mask(&self) -> VulkanImageAspectFlags {
        match self {
            FrameGraphResource::External(external) => external.aspect_mask(),
            FrameGraphResource::Transient(transient) => transient.aspect_mask(),
        }
    }
}
