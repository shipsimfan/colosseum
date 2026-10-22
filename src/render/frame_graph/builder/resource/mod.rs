use crate::render::frame_graph::{FrameGraphExternalResourceInfo, FrameGraphTransientResourceInfo};

mod set;
mod transition;

/// A reference to the builder info for a resource
pub(in crate::render::frame_graph::builder) enum FrameGraphBuilderResource<'a> {
    /// A reference to an external resource's builder info
    External(&'a mut FrameGraphExternalResourceInfo),

    /// A reference to a transient resource's builder info
    Transient(&'a mut FrameGraphTransientResourceInfo),
}
