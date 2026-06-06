use crate::render::frame_graph::{Arenable, FrameGraphResourceState};
use alexandria::{gpu::VulkanImageView, math::Vector2u};

mod get;
mod new;
mod transition;

/// An external resource that is not managed by the frame graph, but can be used as an input or
/// output of a render pass
pub(in crate::render::frame_graph) struct FrameGraphExternalResource<'a> {
    /// The image view of the resource
    image_view: &'a VulkanImageView,

    /// The size of the resource, in pixels
    size: Vector2u,

    /// The state of the resource, used during compilation
    state: FrameGraphResourceState,
}

impl Arenable for FrameGraphExternalResource<'_> {
    type T<'a> = FrameGraphExternalResource<'a>;
}
