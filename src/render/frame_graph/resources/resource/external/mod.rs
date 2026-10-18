use alexandria::{
    gpu::{VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};
use std::sync::Arc;

mod get;
mod new;

/// An external resource used within the frame graph
pub(in crate::render::frame_graph) struct FrameGraphExternalResource {
    /// The image view for the resource
    image_view: Arc<VulkanImageView>,

    /// The size of the resource
    size: Vector2u,

    /// The number of layers in the resource
    layer_count: u32,

    /// The aspect mask of the resource
    aspect_mask: VulkanImageAspectFlags,
}
