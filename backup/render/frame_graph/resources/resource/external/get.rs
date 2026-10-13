use crate::render::frame_graph::{FrameGraphExternalResource, FrameGraphResourceState};
use alexandria::{
    gpu::{VulkanAttachmentLoadOp, VulkanImage, VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};

impl<'a> FrameGraphExternalResource<'a> {
    /// Get the size of the resource, in pixels
    pub(in crate::render::frame_graph::resources) fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the image associated with this resource
    pub(in crate::render::frame_graph::resources::resource) fn image(&self) -> &'a VulkanImage {
        self.image
    }

    /// Get the image view for a resource
    pub(in crate::render::frame_graph::resources::resource) fn image_view(
        &self,
    ) -> &'a VulkanImageView {
        self.image_view
    }

    /// Get the aspect mask for a resource
    pub(in crate::render::frame_graph::resources::resource) fn aspect_mask(
        &self,
    ) -> VulkanImageAspectFlags {
        self.aspect_mask
    }

    /// Get the number of array layers for this resource
    pub(in crate::render::frame_graph::resources::resource) fn layer_count(&self) -> u32 {
        self.layer_count
    }

    /// Get the load operation to use for this resource
    pub(in crate::render::frame_graph::resources) fn load_op(&self) -> VulkanAttachmentLoadOp {
        if unsafe { *self.used.get() } {
            VulkanAttachmentLoadOp::Load
        } else {
            unsafe { *self.used.get() = true };
            VulkanAttachmentLoadOp::Clear
        }
    }

    /// Get the current state of the resource
    pub(in crate::render::frame_graph) fn state(&self) -> &FrameGraphResourceState {
        &self.state
    }
}
