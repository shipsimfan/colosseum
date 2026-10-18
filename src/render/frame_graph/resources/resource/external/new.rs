use crate::render::frame_graph::FrameGraphExternalResource;
use alexandria::{
    gpu::{VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};
use std::sync::Arc;

impl FrameGraphExternalResource {
    /// Create a new external resource
    pub fn new(
        image_view: Arc<VulkanImageView>,
        size: Vector2u,
        layer_count: u32,
        aspect_mask: VulkanImageAspectFlags,
    ) -> FrameGraphExternalResource {
        FrameGraphExternalResource {
            image_view,
            size,
            layer_count,
            aspect_mask,
        }
    }
}
