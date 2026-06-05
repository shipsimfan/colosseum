mod id;
mod pool;
mod resource;

mod get;
mod index;
mod new;
mod register;

pub(in crate::render) use id::*;
pub(in crate::render) use pool::*;

pub(in crate::render::frame_graph) use resource::*;

/// The resources that are available to the frame graph, which can be accessed by nodes during
/// execution
pub(in crate::render::frame_graph) struct FrameGraphResources<'a> {
    /// The resources that have been created by the frame graph
    transient: &'a mut Vec<FrameGraphResource<'a>>,

    /// The external resources that are available to the frame graph, such as the swapchain image
    external: &'a mut Vec<FrameGraphResource<'a>>,
}
