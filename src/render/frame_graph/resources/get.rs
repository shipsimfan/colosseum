use crate::render::frame_graph::{
    FrameGraphExternalResource, FrameGraphResource, FrameGraphResourceId, FrameGraphResources,
};

impl<'a> FrameGraphResources<'a> {
    /// Get a reference to a resource by its ID
    pub fn get<'b>(&'b self, id: FrameGraphResourceId) -> FrameGraphResource<'a, 'b> {
        if id.is_external() {
            FrameGraphResource::External(self.get_external(id))
        } else {
            todo!("transient resources are not yet implemented")
        }
    }

    /// Get a reference to an external resource by its ID
    pub fn get_external<'b>(
        &'b self,
        id: FrameGraphResourceId,
    ) -> &'b FrameGraphExternalResource<'a> {
        debug_assert!(id.is_external());
        &self.external[id.index()]
    }
}
