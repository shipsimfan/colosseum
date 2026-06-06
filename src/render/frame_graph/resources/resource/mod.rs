mod external;
mod state;
mod usage;

mod get;

pub(in crate::render::frame_graph) use external::*;
pub(in crate::render::frame_graph) use state::*;
pub(in crate::render::frame_graph) use usage::*;

/// A resource in the frame graph
pub(in crate::render::frame_graph) enum FrameGraphResource<'a, 'b> {
    /// The resource is external to the frame graph, such as a swapchain image
    External(&'b FrameGraphExternalResource<'a>),
}
