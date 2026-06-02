use crate::render::frame_graph::resources::pool::FrameGraphResourcesPool;

impl FrameGraphResourcesPool {
    /// Reset the resources, clearing all registered resources and allowing them to be registered
    /// again
    pub(in crate::render::frame_graph::resources::pool) fn reset(&mut self) {
        self.external.clear();
        self.transient.clear();
    }
}
