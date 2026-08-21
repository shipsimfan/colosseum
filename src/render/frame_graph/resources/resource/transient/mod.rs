use alexandria::{
    gpu::{VulkanImage, VulkanImageAspectFlags, VulkanImageView, VulkanMemoryRequirements},
    math::Vector2u,
};
use std::cell::UnsafeCell;

mod bind_memory;
mod get;
mod new;
mod reset;

/// A transient resource in the frame graph
pub(in crate::render::frame_graph) struct FrameGraphTransientResource {
    /// The size of the resource, in pixels
    size: Vector2u,

    /// The image itself
    image: VulkanImage,

    /// The image view of the image
    image_view: Option<VulkanImageView>,

    /// The memory requirements of the image
    memory_requirements: VulkanMemoryRequirements,

    /// The aspect mask of the image
    aspect_mask: VulkanImageAspectFlags,

    /// Has the resource been used in this frame?
    used: UnsafeCell<bool>,
}
