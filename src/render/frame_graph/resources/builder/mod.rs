use crate::render::frame_graph::{Arena, FrameGraphExternalResource};

mod finish;
mod new;

/// A builder for creating resources in the frame graph
pub(in crate::render::frame_graph) struct FrameGraphResourceBuilder<'a> {
    /// The external resources that are available to the frame graph, such as the swapchain image
    external: Arena<'a, FrameGraphExternalResource<'a>>,
}
