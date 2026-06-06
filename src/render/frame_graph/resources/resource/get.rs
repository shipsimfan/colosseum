use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceState};
use alexandria::{
    gpu::{VulkanImage, VulkanImageView},
    math::Vector2u,
};

impl<'a, 'b> FrameGraphResource<'a, 'b> {
    /// Get the image view associated with this resource
    pub fn image_view(&self) -> &'a VulkanImageView {
        match self {
            FrameGraphResource::External(external) => external.image_view(),
        }
    }

    /// Get the image associated with this resource
    pub fn image(&self) -> &'a VulkanImage {
        self.image_view().image()
    }

    /// Get the size of the resource
    pub fn size(&self) -> Vector2u {
        match self {
            FrameGraphResource::External(external) => external.size(),
        }
    }

    /// Get the current state of the resource
    pub fn state(&self) -> &FrameGraphResourceState {
        match self {
            FrameGraphResource::External(external) => external.state(),
        }
    }
}
