use crate::render::frame_graph::{FrameGraphResource, FrameGraphResourceId, FrameGraphResources};

impl<'a> FrameGraphResources<'a> {
    /// Get a reference to a resource by its ID
    pub fn get(&self, id: FrameGraphResourceId) -> Option<&FrameGraphResource<'a>> {
        if id.is_external() {
            self.get_external(id)
        } else {
            self.get_transient(id)
        }
    }

    /// Get a reference to an external resource by its ID
    pub fn get_external(&self, id: FrameGraphResourceId) -> Option<&FrameGraphResource<'a>> {
        assert!(id.is_external());

        self.external.get(id.index())
    }

    /// Get a reference to a transient resource by its ID
    pub fn get_transient(&self, id: FrameGraphResourceId) -> Option<&FrameGraphResource<'a>> {
        assert!(id.is_transient());

        self.transient.get(id.index())
    }
}
