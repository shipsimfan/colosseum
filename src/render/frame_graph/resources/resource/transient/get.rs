use crate::render::frame_graph::FrameGraphTransientResource;
use alexandria::{
    gpu::{
        VulkanAttachmentLoadOp, VulkanImage, VulkanImageAspectFlags, VulkanImageView,
        VulkanMemoryRequirements,
    },
    math::Vector2u,
};

impl FrameGraphTransientResource {
    /// Get the size of the resource, in pixels
    pub(in crate::render::frame_graph::resources::resource) fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the image associated with this resource
    pub(in crate::render::frame_graph::resources::resource) fn image(&self) -> &VulkanImage {
        &self.image
    }

    /// Get the image view for a resource
    pub(in crate::render::frame_graph::resources::resource) fn image_view(
        &self,
    ) -> &VulkanImageView {
        self.image_view.as_ref().unwrap()
    }

    /// Get the memory requirements for this resource
    pub fn memory_requirements(&self) -> &VulkanMemoryRequirements {
        &self.memory_requirements
    }

    /// Get the aspect mask for a resource
    pub(in crate::render::frame_graph::resources::resource) fn aspect_mask(
        &self,
    ) -> VulkanImageAspectFlags {
        self.aspect_mask
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
}
