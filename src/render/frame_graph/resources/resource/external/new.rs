use crate::render::frame_graph::{FrameGraphExternalResource, FrameGraphResourceState};
use alexandria::{
    gpu::{VulkanImage, VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};
use std::cell::UnsafeCell;

impl<'a> FrameGraphExternalResource<'a> {
    /// Create a new [`FrameGraphExternalResource`]
    pub(in crate::render::frame_graph::resources) fn new<A: Into<VulkanImageAspectFlags>>(
        size: Vector2u,
        image: &'a VulkanImage,
        image_view: &'a VulkanImageView,
        aspect_mask: A,
    ) -> FrameGraphExternalResource<'a> {
        FrameGraphExternalResource {
            size,
            image,
            image_view,
            aspect_mask: aspect_mask.into(),
            state: FrameGraphResourceState::default(),
            used: UnsafeCell::new(false),
        }
    }
}
