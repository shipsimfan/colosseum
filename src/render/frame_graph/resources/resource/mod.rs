mod external;

mod from;
mod get;

pub(in crate::render::frame_graph) use external::*;

/// A single resource within the frame graph
pub(in crate::render::frame_graph) enum FrameGraphResource<'a> {
    /// An external resource
    External(&'a FrameGraphExternalResource),
}
