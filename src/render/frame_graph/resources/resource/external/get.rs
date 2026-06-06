use crate::render::frame_graph::{FrameGraphExternalResource, FrameGraphResourceState};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl<'a> FrameGraphExternalResource<'a> {
    /// Get the image view of the resource
    pub(in crate::render::frame_graph::resources::resource) fn image_view(
        &self,
    ) -> &'a VulkanImageView {
        self.image_view
    }

    /// Get the size of the resource, in pixels
    pub(in crate::render::frame_graph::resources::resource) fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the current state of the resource
    pub(in crate::render::frame_graph::resources::resource) fn state(
        &self,
    ) -> &FrameGraphResourceState {
        &self.state
    }
}
