use crate::render::frame_graph::{FrameGraphResources, resources::pool::FrameGraphResourcesPool};

impl FrameGraphResourcesPool {
    /// Begin a new [`FrameGraphResources`] for a frame
    pub(in crate::render::frame_graph) fn begin<'a>(&'a mut self) -> FrameGraphResources<'a> {
        self.reset();

        FrameGraphResources::new(
            unsafe { std::mem::transmute(&mut self.transient) },
            unsafe { std::mem::transmute(&mut self.external) },
        )
    }
}
