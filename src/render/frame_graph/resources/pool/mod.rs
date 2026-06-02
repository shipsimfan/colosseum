use crate::render::frame_graph::FrameGraphResource;

mod begin;
mod new;
mod reset;

/// A pool for maintaining the vector's memory allocation across frames, without needing an
/// explicit lifetime marker
pub(in crate::render) struct FrameGraphResourcesPool {
    /// The resources that have been created by the frame graph
    transient: Vec<FrameGraphResource<'static>>,

    /// The external resources that are available to the frame graph, such as the swapchain image
    external: Vec<FrameGraphResource<'static>>,
}
