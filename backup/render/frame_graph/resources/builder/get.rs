use crate::render::frame_graph::{
    FrameGraphExternalResource, FrameGraphResourceBuilder, FrameGraphResourceId,
};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Get an external resource from the builder by its ID
    pub fn get_external(&self, id: FrameGraphResourceId) -> &FrameGraphExternalResource<'a> {
        debug_assert!(id.is_external());
        &self.external[id.index()]
    }
}
