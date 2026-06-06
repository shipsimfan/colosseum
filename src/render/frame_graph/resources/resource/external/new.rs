use crate::render::frame_graph::{FrameGraphExternalResource, FrameGraphResourceState};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

impl<'a> FrameGraphExternalResource<'a> {
    /// Create a new [`ExternalFrameGraphResource`]
    pub(in crate::render::frame_graph::resources) fn new(
        image_view: &'a VulkanImageView,
        size: Vector2u,
    ) -> FrameGraphExternalResource<'a> {
        FrameGraphExternalResource {
            image_view: image_view.into(),
            size,
            state: FrameGraphResourceState::default(),
        }
    }
}
