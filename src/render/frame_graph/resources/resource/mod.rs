use alexandria::{
    gpu::{VulkanAccessFlags, VulkanFormat, VulkanImageLayout, VulkanPipelineStageFlags},
    math::Vector2u,
};
use kind::FrameGraphResourceKind;

mod kind;

mod barrier;
mod deref;
mod get;
mod new;

/// A single resource accessible to nodes in the frame graph, which can be used as an input or output for a node
pub(in crate::render::frame_graph) struct FrameGraphResource<'a> {
    /// The image view of the resource
    image_view: FrameGraphResourceKind<'a>,

    /// The size of the resource, in pixels
    size: Vector2u,

    /// The format of the resource
    format: VulkanFormat,

    /// The current stage mask for the resource
    stage_mask: VulkanPipelineStageFlags,

    /// The current access mask for the resource
    access_mask: VulkanAccessFlags,

    /// The current layout of the resource
    layout: VulkanImageLayout,
}
