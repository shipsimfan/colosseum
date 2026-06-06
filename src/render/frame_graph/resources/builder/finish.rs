use crate::render::frame_graph::{FrameGraphResourceBuilder, FrameGraphResources};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Finish building and return the finalized resources
    pub(in crate::render::frame_graph) fn finish(self) -> FrameGraphResources<'a> {
        FrameGraphResources {
            external: self.external,
        }
    }
}
