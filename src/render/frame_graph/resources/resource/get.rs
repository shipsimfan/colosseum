use crate::render::frame_graph::FrameGraphResource;
use alexandria::{
    gpu::{VulkanImage, VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};

impl<'a> FrameGraphResource<'a> {
    /// Get the image view of the resource
    pub fn image_view(&self) -> &'a VulkanImageView {
        match self {
            FrameGraphResource::External(external) => external.image_view(),
            FrameGraphResource::Transient { image_view, .. } => *image_view,
        }
    }

    /// Get the image of the resource
    pub fn image(&self) -> &'a VulkanImage {
        self.image_view().image()
    }

    /// Get the size of the resource
    pub fn size(&self) -> Vector2u {
        match self {
            FrameGraphResource::External(external) => external.size(),
            FrameGraphResource::Transient { size, .. } => *size,
        }
    }

    /// Get the number of layers of the resource
    pub fn layer_count(&self) -> u32 {
        match self {
            FrameGraphResource::External(external) => external.layer_count(),
            FrameGraphResource::Transient { .. } => 1,
        }
    }

    /// Get the aspect mask of the resource
    pub fn aspect_mask(&self) -> VulkanImageAspectFlags {
        match self {
            FrameGraphResource::External(external) => external.aspect_mask(),
            FrameGraphResource::Transient { info, .. } => info.aspect_mask(),
        }
    }
}
