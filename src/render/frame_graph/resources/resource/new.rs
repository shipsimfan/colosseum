use crate::render::frame_graph::FrameGraphResource;
use alexandria::{
    gpu::{VulkanFormat, VulkanImageLayout, VulkanImageView, VulkanPipelineStageFlag},
    math::Vector2u,
};

impl<'a> FrameGraphResource<'a> {
    /// Create a new [`FrameGraphResource`] for an external resource, such as the swapchain image
    pub(in crate::render::frame_graph::resources) fn new_external(
        image_view: &'a VulkanImageView,
        size: Vector2u,
        format: VulkanFormat,
    ) -> FrameGraphResource<'a> {
        FrameGraphResource {
            image_view: image_view.into(),
            size,
            format,
            stage_mask: VulkanPipelineStageFlag::ColorAttachmentOutput.into(),
            access_mask: 0.into(),
            layout: VulkanImageLayout::Undefined,
        }
    }
}
