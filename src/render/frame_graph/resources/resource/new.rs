use crate::render::frame_graph::{
    FrameGraphResource, resources::resource::FrameGraphResourceState,
};
use alexandria::{
    gpu::{VulkanFormat, VulkanImageView},
    math::Vector2u,
};
use std::cell::RefCell;

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
            state: RefCell::new(FrameGraphResourceState::new()),
        }
    }
}
