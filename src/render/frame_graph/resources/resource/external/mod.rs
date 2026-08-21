use crate::render::frame_graph::{Arenable, FrameGraphResourceState};
use alexandria::{
    gpu::{VulkanImage, VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};
use std::cell::UnsafeCell;

mod get;
mod new;
mod reset;
mod transition;

/// An external resource that is not managed by the frame graph, but can be used as an input or
/// output of a render pass
pub(in crate::render::frame_graph) struct FrameGraphExternalResource<'a> {
    /// The size of the resource, in pixels
    size: Vector2u,

    /// The image associated with the resource
    image: &'a VulkanImage,

    /// The image view of the resource
    image_view: &'a VulkanImageView,

    /// The aspect mask of the resource
    aspect_mask: VulkanImageAspectFlags,

    /// The current state of the resource
    state: FrameGraphResourceState,

    /// Has the resource been used this frame?
    used: UnsafeCell<bool>,
}

impl Arenable for FrameGraphExternalResource<'_> {
    type T<'a> = FrameGraphExternalResource<'a>;
}
