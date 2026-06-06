use crate::render::frame_graph::Arena;

mod builder;
mod id;
mod resource;
mod transition;

mod get;

pub(in crate::render::frame_graph) use builder::*;
pub(in crate::render::frame_graph) use id::*;
pub(in crate::render::frame_graph) use resource::*;

/// The resources that are available to the frame graph, which can be accessed by nodes during
/// execution
pub(in crate::render::frame_graph) struct FrameGraphResources<'a> {
    /// The external resources that are available to the frame graph, such as the swapchain image
    external: Arena<'a, FrameGraphExternalResource<'a>>,
}
