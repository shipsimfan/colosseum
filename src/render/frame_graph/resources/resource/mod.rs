use alexandria::{gpu::VulkanFormat, math::Vector2u};
use kind::FrameGraphResourceKind;
use state::FrameGraphResourceState;
use std::cell::RefCell;

mod kind;
mod state;

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

    /// The current state of the resource
    state: RefCell<FrameGraphResourceState>,
}
